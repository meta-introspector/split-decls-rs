macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl BitAnd for Block { type Output = Block ; # [inline] fn bitand (self , other : Self) -> Self :: Output { Self (self . 0 . bitand (other . 0)) } }
    };
}

impl_4!()