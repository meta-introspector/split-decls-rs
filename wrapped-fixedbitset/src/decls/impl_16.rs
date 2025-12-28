macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl BitOr for Block { type Output = Block ; # [inline] fn bitor (self , other : Self) -> Self :: Output { unsafe { Self (_mm_or_si128 (self . 0 , other . 0)) } } }
    };
}

impl_16!();