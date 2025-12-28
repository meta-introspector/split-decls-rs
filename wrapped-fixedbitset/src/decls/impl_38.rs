macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl BitOr for Block { type Output = Block ; # [inline] fn bitor (self , other : Self) -> Self :: Output { unsafe { Self (_mm256_or_si256 (self . 0 , other . 0)) } } }
    };
}

impl_38!();