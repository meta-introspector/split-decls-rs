// Generated macro for crc32 (function)
macro_rules! Depcrate_hashcrc32 {
() => {
// Module: crate::hash
// Provides: {"crc32"}
// Dependencies: {}
# [doc = " Compute a CRC32 value of the given input `bytes`."] # [doc = ""] # [doc = " In case multiple chunks of `bytes` are present, one should use [`crc32_update()`] instead."] # [cfg (feature = "crc32")] pub fn crc32 (bytes : & [u8]) -> u32 { let mut h = crc32fast :: Hasher :: new () ; h . update (bytes) ; h . finalize () }
};
}
