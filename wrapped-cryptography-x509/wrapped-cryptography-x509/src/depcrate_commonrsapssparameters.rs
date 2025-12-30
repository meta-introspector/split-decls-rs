// Generated macro for RsaPssParameters (struct)
macro_rules! Depcrate_commonRsaPssParameters {
() => {
// Module: crate::common
// Provides: {"RsaPssParameters"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write , Hash , Clone , PartialEq , Eq , Debug)] pub struct RsaPssParameters < 'a > { # [explicit (0)] # [default (PSS_SHA1_HASH_ALG)] pub hash_algorithm : AlgorithmIdentifier < 'a > , # [explicit (1)] # [default (PSS_SHA1_MASK_GEN_ALG)] pub mask_gen_algorithm : MaskGenAlgorithm < 'a > , # [explicit (2)] # [default (20u16)] pub salt_length : u16 , # [explicit (3)] pub _trailer_field : Option < u8 > , }
};
}
