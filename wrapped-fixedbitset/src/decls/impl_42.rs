macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl PartialEq for Block { # [inline] fn eq (& self , other : & Self) -> bool { unsafe { let neq = _mm256_xor_si256 (self . 0 , other . 0) ; _mm256_testz_si256 (neq , neq) == 1 } } }
    };
}

impl_42!()