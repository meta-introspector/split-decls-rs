// Generated macro for tests (module)
macro_rules! Depcrate_pbkdf2tests {
() => {
// Module: crate::pbkdf2
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: pbkdf2 ; use core :: num :: NonZeroU32 ; # [cfg (feature = "fips")] mod fips ; # [test] fn pbkdf2_coverage () { assert ! (pbkdf2 :: PBKDF2_HMAC_SHA256 == pbkdf2 :: PBKDF2_HMAC_SHA256) ; assert ! (pbkdf2 :: PBKDF2_HMAC_SHA256 != pbkdf2 :: PBKDF2_HMAC_SHA384) ; let iterations = NonZeroU32 :: new (100_u32) . unwrap () ; for & alg in & [pbkdf2 :: PBKDF2_HMAC_SHA1 , pbkdf2 :: PBKDF2_HMAC_SHA256 , pbkdf2 :: PBKDF2_HMAC_SHA384 , pbkdf2 :: PBKDF2_HMAC_SHA512 ,] { let mut out = vec ! [0u8 ; 64] ; pbkdf2 :: derive (alg , iterations , b"salt" , b"password" , & mut out) ; let alg_clone = alg ; let mut out2 = vec ! [0u8 ; 64] ; pbkdf2 :: derive (alg_clone , iterations , b"salt" , b"password" , & mut out2) ; assert_eq ! (out , out2) ; } } }
};
}
