macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl BitXorAssign for Block { # [inline] fn bitxor_assign (& mut self , other : Self) { unsafe { self . 0 = _mm_xor_si128 (self . 0 , other . 0) } } }
    };
}

impl_19!();