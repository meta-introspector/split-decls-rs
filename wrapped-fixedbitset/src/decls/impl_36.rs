macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl BitAnd for Block { type Output = Block ; # [inline] fn bitand (self , other : Self) -> Self :: Output { unsafe { Self (_mm256_and_si256 (self . 0 , other . 0)) } } }
    };
}

impl_36!();