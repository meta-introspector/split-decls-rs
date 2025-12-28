macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl BitAndAssign for Block { # [inline] fn bitand_assign (& mut self , other : Self) { unsafe { self . 0 = _mm256_and_pd (self . 0 , other . 0) ; } } }
    };
}

impl_26!();