macro_rules! impl_316 {
    () => {
        impl Add for u128x1_generic { type Output = Self ; # [inline (always)] fn add (self , rhs : Self) -> Self :: Output { omap2 (self , rhs , | x , y | x . wrapping_add (y)) } }
    };
}

impl_316!();