macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl Block { # [inline] pub fn is_empty (self) -> bool { unsafe { _mm256_testz_si256 (self . 0 , self . 0) == 1 } } # [inline] pub fn andnot (self , other : Self) -> Self { Self (unsafe { _mm256_andnot_si256 (other . 0 , self . 0) }) } }
    };
}

impl_34!()