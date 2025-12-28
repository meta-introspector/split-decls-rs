macro_rules! deps {
    () => {
        MlKem512Internal!();
        PkeParameters!();
        FieldElement!();
        UnknownCryptoError!();
        ByteSerialization!();
        RingElement!();
    };
}

macro_rules! impl_462 {
    () => {
        deps!();
        impl PkeParameters for MlKem512Internal { const K : usize = 2 ; const ETA_1 : usize = 3 ; const ETA_2 : usize = 2 ; const D_U : u8 = 10 ; const D_V : u8 = 4 ; const EK_SIZE : usize = 800 ; const DK_SIZE : usize = 1632 ; const CIPHERTEXT_SIZE : usize = 768 ; const SHARED_SECRET_SIZE : usize = 32 ; fn sample_poly_cbd_eta1 (seed : & [u8] , b : u8) -> Result < RingElement , UnknownCryptoError > { let mut prf_out = Zeroizing :: new ([0u8 ; 64 * Self :: ETA_1]) ; let mut bits = Zeroizing :: new ([0u8 ; (64 * Self :: ETA_1) * 8]) ; sample_poly_cbd (seed , b , prf_out . as_mut () , bits . as_mut () , Self :: ETA_1) } fn sample_poly_cbd_eta2 (seed : & [u8] , b : u8) -> Result < RingElement , UnknownCryptoError > { let mut prf_out = Zeroizing :: new ([0u8 ; 64 * Self :: ETA_2]) ; let mut bits = Zeroizing :: new ([0u8 ; (64 * Self :: ETA_2) * 8]) ; sample_poly_cbd (seed , b , prf_out . as_mut () , bits . as_mut () , Self :: ETA_2) } fn encode_dv (coefficients : & [FieldElement] , out : & mut [u8]) { debug_assert_eq ! (out . len () , Self :: ENCODE_SIZE_D_V) ; ByteSerialization :: encode_4 (coefficients , out) ; } fn encode_du (coefficients : & [FieldElement] , out : & mut [u8]) { debug_assert_eq ! (out . len () , Self :: ENCODE_SIZE_D_U) ; ByteSerialization :: encode_10 (coefficients , out) ; } fn decode_dv (inbytes : & [u8] , out : & mut [FieldElement]) { debug_assert_eq ! (inbytes . len () , Self :: ENCODE_SIZE_D_V) ; ByteSerialization :: decode_4 (inbytes , out) ; } fn decode_du (inbytes : & [u8] , out : & mut [FieldElement]) { debug_assert_eq ! (inbytes . len () , Self :: ENCODE_SIZE_D_U) ; ByteSerialization :: decode_10 (inbytes , out) ; } }
    };
}

impl_462!();