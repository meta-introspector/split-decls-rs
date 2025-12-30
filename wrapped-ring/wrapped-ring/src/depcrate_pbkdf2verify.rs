// Generated macro for verify (function)
macro_rules! Depcrate_pbkdf2verify {
() => {
// Module: crate::pbkdf2
// Provides: {"verify"}
// Dependencies: {}
# [doc = " Verifies that a previously-derived (e.g., using `derive`) PBKDF2 value"] # [doc = " matches the PBKDF2 value derived from the other inputs."] # [doc = ""] # [doc = " The comparison is done in constant time to prevent timing attacks. The"] # [doc = " comparison will fail if `previously_derived` is empty (has a length of"] # [doc = " zero)."] # [doc = ""] # [doc = " | Parameter                  | RFC 2898 Section 5.2 Term"] # [doc = " |----------------------------|--------------------------------------------"] # [doc = " | digest_alg                 | PRF (HMAC with the given digest algorithm)."] # [doc = " | `iterations`               | c (iteration count)"] # [doc = " | `salt`                     | S (salt)"] # [doc = " | `secret`                   | P (password)"] # [doc = " | `previously_derived`       | dk (derived key)"] # [doc = " | `previously_derived.len()` | dkLen (derived key length)"] pub fn verify (algorithm : Algorithm , iterations : NonZeroU32 , salt : & [u8] , secret : & [u8] , previously_derived : & [u8] ,) -> Result < () , error :: Unspecified > { let cpu = cpu :: features () ; try_verify (algorithm , iterations , salt , secret , previously_derived , cpu) . map_err (error :: erase :: < VerifyError >) }
};
}
