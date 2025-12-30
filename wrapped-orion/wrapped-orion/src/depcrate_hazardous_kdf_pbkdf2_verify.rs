// Generated macro for _verify (function)
macro_rules! Depcrate_hazardous_kdf_pbkdf2_verify {
() => {
// Module: crate::hazardous::kdf::pbkdf2
// Provides: {"_verify"}
// Dependencies: {}
# [doc = ""] # [doc = ""] # [doc = " NOTE: Hmac has the output size of the hash function defined,"] # [doc = " but the array initialization with the size cannot depend on a generic parameter,"] # [doc = " because we don't have full support for const generics yet."] fn _verify < Hmac , const OUTSIZE : usize > (expected : & [u8] , padded_password : & [u8] , salt : & [u8] , iterations : usize , dest : & mut [u8] ,) -> Result < () , UnknownCryptoError > where Hmac : hmac :: HmacFunction , { debug_assert_eq ! (OUTSIZE , Hmac :: HASH_FUNC_OUTSIZE) ; _derive_key :: < Hmac , { OUTSIZE } > (padded_password , salt , iterations , dest) ? ; crate :: util :: secure_cmp (expected , dest) }
};
}
