macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl BitXor for Block { type Output = Block ; # [inline] fn bitxor (self , other : Self) -> Self :: Output { Self (self . 0 . bitxor (other . 0)) } }
    };
}

impl_8!()