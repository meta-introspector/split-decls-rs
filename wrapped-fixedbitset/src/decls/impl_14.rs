macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl BitAnd for Block { type Output = Block ; # [inline] fn bitand (self , other : Self) -> Self :: Output { unsafe { Self (_mm_and_si128 (self . 0 , other . 0)) } } }
    };
}

impl_14!()