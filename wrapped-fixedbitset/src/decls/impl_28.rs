macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl BitOrAssign for Block { # [inline] fn bitor_assign (& mut self , other : Self) { unsafe { self . 0 = _mm256_or_pd (self . 0 , other . 0) ; } } }
    };
}

impl_28!();