macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl BitXor for Block { type Output = Block ; # [inline] fn bitxor (self , other : Self) -> Self :: Output { Self (v128_xor (self . 0 , other . 0)) } }
    };
}

impl_51!()