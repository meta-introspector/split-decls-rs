macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl BitOr for Block { type Output = Block ; # [inline] fn bitor (self , other : Self) -> Self :: Output { Self (self . 0 . bitor (other . 0)) } }
    };
}

impl_6!()