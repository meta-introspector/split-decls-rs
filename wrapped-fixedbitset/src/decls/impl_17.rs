macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl BitOrAssign for Block { # [inline] fn bitor_assign (& mut self , other : Self) { unsafe { self . 0 = _mm_or_si128 (self . 0 , other . 0) ; } } }
    };
}

impl_17!();