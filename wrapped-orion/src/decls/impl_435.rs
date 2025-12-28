macro_rules! deps {
    () => {
        RingElementNTT!();
        FieldElement!();
        RingElement!();
    };
}

macro_rules! impl_435 {
    () => {
        deps!();
        impl RingElementNTT { pub fn zero () -> Self { Self { coefficients : [FieldElement :: zero () ; 256] , } } # [doc = " NOTE: This should not be accessible by a user."] pub (crate) fn copy_from_non_ntt (not_ntt : & RingElement) -> Self { Self { coefficients : not_ntt . coefficients , } } # [doc = " FIPS-203, Algorithm 12."] pub fn base_case_multiply (a0 : FieldElement , a1 : FieldElement , b0 : FieldElement , b1 : FieldElement , gamma : FieldElement ,) -> (FieldElement , FieldElement) { let c0 : FieldElement = a0 * b0 + a1 * b1 * gamma ; let c1 : FieldElement = a0 * b1 + a1 * b0 ; (c0 , c1) } }
    };
}

impl_435!();