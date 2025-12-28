macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl PartialEq for Block { # [inline] fn eq (& self , other : & Self) -> bool { unsafe { let new = _mm256_xor_pd (self . 0 , other . 0) ; let neq = _mm256_castpd_si256 (new) ; _mm256_testz_si256 (neq , neq) == 1 } } }
    };
}

impl_31!();