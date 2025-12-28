macro_rules! deps {
    () => {
        Sha3_256!();
        UnknownCryptoError!();
        FieldElement!();
        ByteSerialization!();
        RingElement!();
    };
}

macro_rules! PkeParameters {
    () => {
        deps!();
        # [doc = " Internal PKE-related function, for generalizing over the three different PKE parameter-sets."] pub (crate) trait PkeParameters { const N : usize = 256 ; const K : usize ; const ETA_1 : usize ; const ETA_2 : usize ; const D_U : u8 ; const D_V : u8 ; # [doc = " Encapsulation key size (bytes)."] const EK_SIZE : usize ; # [doc = " Decapsulation key size (bytes)."] const DK_SIZE : usize ; # [doc = " Ciphertext size (bytes)."] const CIPHERTEXT_SIZE : usize ; # [doc = " Shared Secret size (bytes)."] const SHARED_SECRET_SIZE : usize ; const ENCODE_SIZE_D_U : usize = Self :: N * Self :: D_U as usize / 8 ; const ENCODE_SIZE_D_V : usize = Self :: N * Self :: D_V as usize / 8 ; # [doc = " \"It is important to note that this checking process does not guarantee"] # [doc = " that ek is a properly produced output of ML-KEM.KeyGen.\", p.36."] fn encapsulation_key_check (ek : & [u8]) -> Result < () , UnknownCryptoError > { debug_assert_eq ! (Self :: EK_SIZE , (ENCODE_SIZE_POLY * Self :: K) + 32) ; if ek . len () != Self :: EK_SIZE { return Err (UnknownCryptoError) ; } let mut modulus_check = [FieldElement :: zero () ; KYBER_POLY_DEG] ; let mut modulus_check_bytes = [0u8 ; ENCODE_SIZE_POLY] ; for ek_part in ek . chunks_exact (ENCODE_SIZE_POLY) . take (Self :: K) { ByteSerialization :: decode_12 (ek_part , & mut modulus_check) ; ByteSerialization :: encode_12 (& modulus_check , & mut modulus_check_bytes) ; if modulus_check_bytes != ek_part { return Err (UnknownCryptoError) ; } } Ok (()) } # [doc = " NOTE: the Decapsulation input check, Check 1. is not included in this function on purpose."] # [doc = " The Ciphertext newtype is bound by it's length, so that check is"] # [doc = " automatically a part of that newtype."] fn decapsulation_key_check (dk : & [u8]) -> Result < () , UnknownCryptoError > { if dk . len () != Self :: DK_SIZE { return Err (UnknownCryptoError) ; } let hash_check = Sha3_256 :: digest (& dk [ENCODE_SIZE_POLY * Self :: K .. (768 * Self :: K) + 32]) ? ; if bool :: from (hash_check . as_ref () . ct_ne (& dk [(768 * Self :: K) + 32 .. (768 * Self :: K) + 64]) ,) { return Err (UnknownCryptoError) ; } Ok (()) } fn sample_poly_cbd_eta1 (seed : & [u8] , b : u8) -> Result < RingElement , UnknownCryptoError > ; fn sample_poly_cbd_eta2 (seed : & [u8] , b : u8) -> Result < RingElement , UnknownCryptoError > ; fn encode_dv (coefficients : & [FieldElement] , out : & mut [u8]) ; fn encode_du (coefficients : & [FieldElement] , out : & mut [u8]) ; fn decode_dv (inbytes : & [u8] , out : & mut [FieldElement]) ; fn decode_du (inbytes : & [u8] , out : & mut [FieldElement]) ; }
    };
}

PkeParameters!();