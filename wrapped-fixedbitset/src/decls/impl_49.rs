macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl BitOr for Block { type Output = Block ; # [inline] fn bitor (self , other : Self) -> Self :: Output { Self (v128_or (self . 0 , other . 0)) } }
    };
}

impl_49!();