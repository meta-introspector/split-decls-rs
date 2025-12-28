macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl BitAndAssign for Block { # [inline] fn bitand_assign (& mut self , other : Self) { unsafe { self . 0 = _mm_and_si128 (self . 0 , other . 0) ; } } }
    };
}

impl_15!();