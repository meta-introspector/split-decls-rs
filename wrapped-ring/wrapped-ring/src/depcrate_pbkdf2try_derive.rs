// Generated macro for try_derive (function)
macro_rules! Depcrate_pbkdf2try_derive {
() => {
// Module: crate::pbkdf2
// Provides: {"try_derive"}
// Dependencies: {}
fn try_derive (algorithm : Algorithm , iterations : NonZeroU32 , salt : & [u8] , secret : & [u8] , out : & mut [u8] , cpu : cpu :: Features ,) -> Result < () , DeriveError > { let digest_alg = algorithm . 0 . digest_algorithm () ; let output_len = digest_alg . output_len () ; let secret = hmac :: Key :: try_new (algorithm . 0 , secret , cpu) . map_err (DeriveError :: secret_too_long) ? ; out . fill (0) ; let mut idx : u32 = 0 ; let out_len = out . len () ; for chunk in out . chunks_mut (output_len) { idx = idx . checked_add (1) . ok_or_else (| | { DeriveError :: too_much_output_requested (TooMuchOutputRequestedError :: new (out_len)) }) ? ; derive_block (& secret , iterations , salt , idx , chunk , cpu) . map_err (DeriveError :: salt_too_long) ? ; } Ok (()) }
};
}
