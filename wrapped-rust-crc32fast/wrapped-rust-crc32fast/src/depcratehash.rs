// Generated macro for hash (function)
macro_rules! Depcratehash {
() => {
// Module: crate
// Provides: {"hash"}
// Dependencies: {}
# [doc = " Computes the CRC32 hash of a byte slice."] # [doc = ""] # [doc = " Check out [`Hasher`] for more advanced use-cases."] pub fn hash (buf : & [u8]) -> u32 { let mut h = Hasher :: new () ; h . update (buf) ; h . finalize () }
};
}
