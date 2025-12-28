macro_rules! deps {
    () => {
        FieldElement!();
        RingElementNTT!();
        RingElement!();
    };
}

macro_rules! impl_428 {
    () => {
        deps!();
        impl RingElement { pub fn zero () -> Self { Self { coefficients : [FieldElement :: zero () ; 256] , } } # [doc = " NOTE: This should not be accessible by a user."] pub (crate) fn copy_from_ntt (ntt : & RingElementNTT) -> Self { Self { coefficients : ntt . coefficients , } } # [cfg (all (test , feature = "safe_api"))] pub (crate) fn random_element () -> Self { use crate :: hazardous :: kem :: ml_kem :: internal :: fe :: KYBER_Q ; use rand :: { prelude :: * , rng } ; let mut rng = rng () ; let mut coefficients = [FieldElement :: zero () ; 256] ; for rand_coeff in coefficients . iter_mut () { let new = rng . random_range (0 .. KYBER_Q) ; * rand_coeff = FieldElement :: new (new) ; } Self { coefficients } } }
    };
}

impl_428!();