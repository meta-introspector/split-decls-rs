macro_rules! impl_314 {
    () => {
        impl Add for u32x4_generic { type Output = Self ; # [inline (always)] fn add (self , rhs : Self) -> Self :: Output { dmap2 (self , rhs , | x , y | x . wrapping_add (y)) } }
    };
}

impl_314!();