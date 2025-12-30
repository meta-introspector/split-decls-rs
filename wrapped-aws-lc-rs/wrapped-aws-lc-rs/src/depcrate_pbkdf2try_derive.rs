// Generated macro for try_derive (function)
macro_rules! Depcrate_pbkdf2try_derive {
() => {
// Module: crate::pbkdf2
// Provides: {"try_derive"}
// Dependencies: {}
# [inline] fn try_derive (algorithm : Algorithm , iterations : NonZeroU32 , salt : & [u8] , secret : & [u8] , out : & mut [u8] ,) -> Result < () , Unspecified > { assert ! (out . len () as u64 <= algorithm . max_output_len , "derived key too long") ; if 1 != indicator_check ! (unsafe { PKCS5_PBKDF2_HMAC (secret . as_ptr () . cast () , secret . len () , salt . as_ptr () , salt . len () , iterations . get () , * digest :: match_digest_type (& algorithm . algorithm . digest_algorithm () . id) , out . len () , out . as_mut_ptr () ,) }) { return Err (Unspecified) ; } Ok (()) }
};
}
