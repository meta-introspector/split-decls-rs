// Generated macro for impl_252 (impl)
macro_rules! Depcrate_digestimpl_252 {
() => {
// Module: crate::digest
// Provides: {"impl_252"}
// Dependencies: {}
impl Algorithm { # [doc = " The internal block length."] pub fn block_len (& self) -> usize { self . block_len . into () } # [doc = " The size of the chaining value of the digest function, in bytes."] # [doc = ""] # [doc = " For non-truncated algorithms (SHA-1, SHA-256, SHA-512), this is equal"] # [doc = " to [`Self::output_len()`]. For truncated algorithms (e.g. SHA-384,"] # [doc = " SHA-512/256), this is equal to the length before truncation. This is"] # [doc = " mostly helpful for determining the size of an HMAC key that is"] # [doc = " appropriate for the digest algorithm."] pub fn chaining_len (& self) -> usize { self . chaining_len } # [doc = " The length of a finalized digest."] pub fn output_len (& self) -> usize { self . output_len . into () } }
};
}
