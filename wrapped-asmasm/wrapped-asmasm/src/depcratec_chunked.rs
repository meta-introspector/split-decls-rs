// Generated macro for c_chunked (function)
macro_rules! Depcratec_chunked {
() => {
// Module: crate
// Provides: {"c_chunked"}
// Dependencies: {}
# [inline (never)] fn c_chunked (file : & [u8] , chunk_size : usize) -> u64 { let mut hasher = C :: new () ; for chunk in file . chunks (chunk_size) { hasher . write (chunk) ; } hasher . finish () }
};
}
