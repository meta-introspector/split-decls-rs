// Generated macro for impl_192 (impl)
macro_rules! Depcrate_digestimpl_192 {
() => {
// Module: crate::digest
// Provides: {"impl_192"}
// Dependencies: {}
impl Algorithm { # [doc = " The length of a finalized digest."] # [inline] # [must_use] pub fn output_len (& self) -> usize { self . output_len } # [doc = " The size of the chaining value of the digest function, in bytes. For"] # [doc = " non-truncated algorithms (SHA-1, SHA-256, SHA-512), this is equal to"] # [doc = " `output_len`. For truncated algorithms (e.g. SHA-224, SHA-384, SHA-512/256),"] # [doc = " this is equal to the length before truncation. This is mostly helpful"] # [doc = " for determining the size of an HMAC key that is appropriate for the"] # [doc = " digest algorithm."] # [doc = ""] # [doc = " This function isn't actually used in *aws-lc-rs*, and is only"] # [doc = " kept for compatibility with the original *ring* implementation."] # [deprecated] # [inline] # [must_use] pub fn chaining_len (& self) -> usize { # ! [allow (deprecated)] self . chaining_len } # [doc = " The internal block length."] # [inline] # [must_use] pub fn block_len (& self) -> usize { self . block_len } }
};
}
