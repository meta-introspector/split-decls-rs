// Generated macro for Algorithm (trait)
macro_rules! Depcrate_digestAlgorithm {
() => {
// Module: crate::digest
// Provides: {"Algorithm"}
// Dependencies: {}
# [doc = " Provides the ability to hash in an algorithm-agnostic manner."] pub trait Algorithm { # [doc = " The size of the resulting digest."] const OUTPUT_LEN : usize ; # [doc = " The block length (in bytes)."] const BLOCK_LEN : usize ; # [doc = " Gets a reference to a message digest algorithm to be used by the HKDF implementation."] # [doc (hidden)] fn get_md (_ : sealed :: Sealed) -> & 'static MdRef ; # [doc = " Hashes a message."] fn hash_to_vec (input : & [u8]) -> Vec < u8 > ; # [doc = " Create a new context for incremental hashing."] fn new () -> Self ; # [doc = " Hash the contents of `input`."] fn update (& mut self , input : & [u8]) ; # [doc = " Finish the hashing and return the digest."] fn digest_to_vec (self) -> Vec < u8 > ; }
};
}
