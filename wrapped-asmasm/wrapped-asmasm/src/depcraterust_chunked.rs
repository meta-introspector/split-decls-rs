// Generated macro for rust_chunked (function)
macro_rules! Depcraterust_chunked {
() => {
// Module: crate
// Provides: {"rust_chunked"}
// Dependencies: {}
# [inline (never)] fn rust_chunked (file : & [u8] , chunk_size : usize) -> u64 { let mut hasher = XxHash3_64 :: new () ; for chunk in file . chunks (chunk_size) { hasher . write (chunk) ; } hasher . finish () }
};
}
