// Generated macro for InsecureSha1 (struct)
macro_rules! Depcrate_digestInsecureSha1 {
() => {
// Module: crate::digest
// Provides: {"InsecureSha1"}
// Dependencies: {}
# [doc = " The insecure SHA-1 hash algorithm."] # [doc = ""] # [doc = " Some existing protocols depend on SHA-1 and so it is provided here, but it"] # [doc = " does not provide collision resistance and should not be used if at all"] # [doc = " avoidable. Use SHA-256 instead."] # [derive (Clone)] pub struct InsecureSha1 { ctx : bssl_sys :: SHA_CTX , }
};
}
