// Generated macro for Algorithm (struct)
macro_rules! Depcrate_digestAlgorithm {
() => {
// Module: crate::digest
// Provides: {"Algorithm"}
// Dependencies: {}
# [doc = " A digest algorithm."] pub struct Algorithm { # [doc = " The length of a finalized digest."] pub output_len : usize , # [doc = " The size of the chaining value of the digest function, in bytes. For"] # [doc = " non-truncated algorithms (SHA-1, SHA-256, SHA-512), this is equal to"] # [doc = " `output_len`. For truncated algorithms (e.g. SHA-224, SHA-384, SHA-512/256),"] # [doc = " this is equal to the length before truncation. This is mostly helpful"] # [doc = " for determining the size of an HMAC key that is appropriate for the"] # [doc = " digest algorithm."] # [doc = ""] # [doc = " This function isn't actually used in *aws-lc-rs*, and is only"] # [doc = " kept for compatibility with the original *ring* implementation."] # [deprecated] pub chaining_len : usize , # [doc = " The internal block length."] pub block_len : usize , max_input_len : u64 , one_shot_hash : fn (msg : & [u8] , output : & mut [u8]) , pub (crate) id : AlgorithmID , }
};
}
