macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl BitAnd for Block { type Output = Block ; # [inline] fn bitand (self , other : Self) -> Self :: Output { Self (v128_and (self . 0 , other . 0)) } }
    };
}

impl_47!();