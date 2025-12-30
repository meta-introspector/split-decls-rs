// Generated macro for truncated_md5 (function)
macro_rules! Depcrate_llvm_utilstruncated_md5 {
() => {
// Module: crate::llvm_utils
// Provides: {"truncated_md5"}
// Dependencies: {}
# [doc = " LLVM's profiler/coverage metadata often uses an MD5 hash truncated to"] # [doc = " 64 bits as a way to associate data stored in different tables/sections."] pub (crate) fn truncated_md5 (bytes : & [u8]) -> u64 { use md5 :: { Digest , Md5 } ; let mut hasher = Md5 :: new () ; hasher . update (bytes) ; let hash : [u8 ; 8] = hasher . finalize () . as_slice () [.. 8] . try_into () . unwrap () ; u64 :: from_le_bytes (hash) }
};
}
