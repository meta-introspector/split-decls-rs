macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl BitXor for Block { type Output = Block ; # [inline] fn bitxor (self , other : Self) -> Self :: Output { unsafe { Self (_mm_xor_si128 (self . 0 , other . 0)) } } }
    };
}

impl_18!()