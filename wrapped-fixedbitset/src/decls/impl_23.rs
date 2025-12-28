macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl Block { # [inline] pub fn is_empty (self) -> bool { unsafe { let value = _mm256_castpd_si256 (self . 0) ; _mm256_testz_si256 (value , value) == 1 } } # [inline] pub fn andnot (self , other : Self) -> Self { unsafe { Self (_mm256_andnot_pd (other . 0 , self . 0)) } } }
    };
}

impl_23!();