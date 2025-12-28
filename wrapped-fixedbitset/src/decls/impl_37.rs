macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl BitAndAssign for Block { # [inline] fn bitand_assign (& mut self , other : Self) { unsafe { self . 0 = _mm256_and_si256 (self . 0 , other . 0) ; } } }
    };
}

impl_37!();