macro_rules! impl_315 {
    () => {
        impl Add for u64x2_generic { type Output = Self ; # [inline (always)] fn add (self , rhs : Self) -> Self :: Output { qmap2 (self , rhs , | x , y | x . wrapping_add (y)) } }
    };
}

impl_315!();