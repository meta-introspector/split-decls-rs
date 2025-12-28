macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl BitXor for Block { type Output = Block ; # [inline] fn bitxor (self , other : Self) -> Self :: Output { unsafe { Self (_mm256_xor_si256 (self . 0 , other . 0)) } } }
    };
}

impl_40!();