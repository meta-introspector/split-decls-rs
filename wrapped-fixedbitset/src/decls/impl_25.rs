macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl BitAnd for Block { type Output = Block ; # [inline] fn bitand (self , other : Self) -> Self :: Output { unsafe { Self (_mm256_and_pd (self . 0 , other . 0)) } } }
    };
}

impl_25!()