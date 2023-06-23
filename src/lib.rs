#![cfg_attr(not(test), no_std)]
#![feature(const_trait_impl)]
#![feature(const_fn_floating_point_arithmetic)]

#[const_trait]
pub trait NumScale<F>
{
    fn scale(self, x: F) -> Self;
}

impl const NumScale<f64> for f64
{
    fn scale(self, x: f64) -> Self
    {
        self*x
    }
}
impl const NumScale<f32> for f64
{
    fn scale(self, x: f32) -> Self
    {
        self*x as f64
    }
}
impl const NumScale<f32> for f32
{
    fn scale(self, x: f32) -> Self
    {
        self*x
    }
}
impl const NumScale<f64> for f32
{
    fn scale(self, x: f64) -> Self
    {
        self*x as f32
    }
}

macro_rules! impl_scale_num {
    ($num:ty) => {
        impl const NumScale<f32> for $num
        {
            fn scale(self, x: f32) -> Self
            {
                (self as f32*x) as Self
            }
        }
        impl const NumScale<f64> for $num
        {
            fn scale(self, x: f64) -> Self
            {
                (self as f64*x) as Self
            }
        }
    };
}

impl_scale_num!(usize);
impl_scale_num!(isize);

impl_scale_num!(u8);
impl_scale_num!(u16);
impl_scale_num!(u32);
impl_scale_num!(u64);
impl_scale_num!(u128);

impl_scale_num!(i8);
impl_scale_num!(i16);
impl_scale_num!(i32);
impl_scale_num!(i64);
impl_scale_num!(i128);

#[cfg(test)]
mod test
{
    use std::{println, fmt::Display, any::type_name};

    use crate::NumScale;

    #[test]
    fn it_works()
    {
        const N: usize = 32;
        let x: Vec<f64> = (0..N).map(|i| i as f64/(N - 1) as f64).collect();
        test_num([u8::MAX].into_iter(), &x);
        println!("u8");
        test_num([u16::MAX].into_iter(), &x);
        println!("u16");
        test_num([u32::MAX].into_iter(), &x);
        println!("u32");
        test_num([u64::MAX].into_iter(), &x);
        println!("u64");
        test_num([u128::MAX].into_iter(), &x);
        println!("u128");
        
        test_num([i8::MIN, i8::MAX].into_iter(), &x);
        println!("i8");
        test_num([i16::MIN, i16::MAX].into_iter(), &x);
        println!("i16");
        test_num([i32::MIN, i32::MAX].into_iter(), &x);
        println!("i32");
        test_num([i64::MIN, i64::MAX].into_iter(), &x);
        println!("i64");
        test_num([i128::MIN, i128::MAX].into_iter(), &x);
        println!("i128");
        
        test_num([usize::MAX].into_iter(), &x);
        println!("usize");
        test_num([isize::MIN, isize::MAX].into_iter(), &x);
        println!("isize");
    }

    fn test_num<I, F>(iter: I, x: &[F])
    where
        F: Copy,
        I: Iterator,
        I::Item: NumScale<F> + Copy + Display
    {
        for n in iter
        {
            for &x in x
            {
                println!("{}{}", n.scale(x), type_name::<I::Item>());
            }
        }
    }
}