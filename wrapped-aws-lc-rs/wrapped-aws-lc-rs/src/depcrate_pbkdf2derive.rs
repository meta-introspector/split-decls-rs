// Generated macro for derive (function)
macro_rules! Depcrate_pbkdf2derive {
() => {
// Module: crate::pbkdf2
// Provides: {"derive"}
// Dependencies: {}
# [doc = " Fills `out` with the key derived using PBKDF2 with the given inputs."] # [doc = ""] # [doc = " Do not use `derive` as part of verifying a secret; use `verify` instead, to"] # [doc = " minimize the effectiveness of timing attacks."] # [doc = ""] # [doc = " `out.len()` must be no larger than the digest length * (2**32 - 1), per the"] # [doc = " PBKDF2 specification."] # [doc = ""] # [doc = " | Parameter   | RFC 2898 Section 5.2 Term"] # [doc = " |-------------|-------------------------------------------"] # [doc = " | `digest_alg`  | PRF (HMAC with the given digest algorithm)"] # [doc = " | `iterations`  | c (iteration count)"] # [doc = " | `salt`        | S (salt)"] # [doc = " | `secret`      | P (password)"] # [doc = " | `out`         | dk (derived key)"] # [doc = " | `out.len()`   | dkLen (derived key length)"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " `derive` panics if `out.len()` is larger than (2**32 - 1) * the digest"] # [doc = " algorithm's output length, per the PBKDF2 specification."] # [inline] pub fn derive (algorithm : Algorithm , iterations : NonZeroU32 , salt : & [u8] , secret : & [u8] , out : & mut [u8] ,) { try_derive (algorithm , iterations , salt , secret , out) . expect ("pbkdf2 derive failed") ; }
};
}
