// Generated macro for _derive_key (function)
macro_rules! Depcrate_hazardous_kdf_hkdf_derive_key {
() => {
// Module: crate::hazardous::kdf::hkdf
// Provides: {"_derive_key"}
// Dependencies: {}
# [doc = " Combine `extract` and `expand` to return a derived key."] # [doc = ""] # [doc = " NOTE: See comment about const param at _extract function."] fn _derive_key < Hmac , const OUTSIZE : usize > (salt : & [u8] , ikm : & [u8] , info : Option < & [u8] > , dest : & mut [u8] ,) -> Result < () , UnknownCryptoError > where Hmac : hmac :: HmacFunction , { _expand :: < Hmac , { OUTSIZE } > (& _extract :: < Hmac , { OUTSIZE } > (salt , ikm) ? , info , dest) }
};
}
