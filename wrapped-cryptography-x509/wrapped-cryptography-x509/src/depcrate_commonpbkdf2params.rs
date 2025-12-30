// Generated macro for PBKDF2Params (struct)
macro_rules! Depcrate_commonPBKDF2Params {
() => {
// Module: crate::common
// Provides: {"PBKDF2Params"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write , PartialEq , Eq , Hash , Clone , Debug)] pub struct PBKDF2Params < 'a > { pub salt : & 'a [u8] , pub iteration_count : u64 , pub key_length : Option < u64 > , # [default (HMAC_SHA1_ALG)] pub prf : Box < AlgorithmIdentifier < 'a > > , }
};
}
