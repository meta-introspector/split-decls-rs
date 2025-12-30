// Generated macro for try_verify (function)
macro_rules! Depcrate_pbkdf2try_verify {
() => {
// Module: crate::pbkdf2
// Provides: {"try_verify"}
// Dependencies: {}
fn try_verify (algorithm : Algorithm , iterations : NonZeroU32 , salt : & [u8] , secret : & [u8] , previously_derived : & [u8] , cpu : cpu :: Features ,) -> Result < () , VerifyError > { let digest_alg = algorithm . 0 . digest_algorithm () ; if previously_derived . is_empty () { return Err (VerifyError :: previously_derived_empty (0)) ; } let mut derived_buf = [0u8 ; digest :: MAX_OUTPUT_LEN] ; let output_len = digest_alg . output_len () ; let secret = hmac :: Key :: try_new (algorithm . 0 , secret , cpu) . map_err (VerifyError :: secret_too_long) ? ; let mut idx : u32 = 0 ; let mut matches = bb :: BoolMask :: TRUE ; for previously_derived_chunk in previously_derived . chunks (output_len) { idx = idx . checked_add (1) . ok_or_else (| | { VerifyError :: mismatch (()) }) ? ; let derived_chunk = & mut derived_buf [.. previously_derived_chunk . len ()] ; derived_chunk . fill (0) ; derive_block (& secret , iterations , salt , idx , derived_chunk , cpu) . map_err (VerifyError :: salt_too_long) ? ; let current_block_matches = bb :: bytes_are_equal (derived_chunk , previously_derived_chunk) ; matches &= current_block_matches ; } if ! matches . leak () { return Err (VerifyError :: mismatch (())) ; } Ok (()) }
};
}
