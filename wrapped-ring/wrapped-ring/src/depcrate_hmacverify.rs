// Generated macro for verify (function)
macro_rules! Depcrate_hmacverify {
() => {
// Module: crate::hmac
// Provides: {"verify"}
// Dependencies: {}
# [doc = " Calculates the HMAC of `data` using the signing key `key`, and verifies"] # [doc = " whether the resultant value equals `tag`, in one step."] # [doc = ""] # [doc = " This is logically equivalent to, but more efficient than, constructing a"] # [doc = " `Key` with the same value as `key` and then using `verify`."] # [doc = ""] # [doc = " The verification will be done in constant time to prevent timing attacks."] pub fn verify (key : & Key , data : & [u8] , tag : & [u8]) -> Result < () , error :: Unspecified > { key . verify (data , tag , cpu :: features ()) . map_err (| _ : VerifyError | error :: Unspecified) }
};
}
