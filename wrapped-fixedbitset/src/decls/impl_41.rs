macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl BitXorAssign for Block { # [inline] fn bitxor_assign (& mut self , other : Self) { unsafe { self . 0 = _mm256_xor_si256 (self . 0 , other . 0) } } }
    };
}

impl_41!();