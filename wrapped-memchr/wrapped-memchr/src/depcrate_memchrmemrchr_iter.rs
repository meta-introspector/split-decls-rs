// Generated macro for memrchr_iter (function)
macro_rules! Depcrate_memchrmemrchr_iter {
() => {
// Module: crate::memchr
// Provides: {"memrchr_iter"}
// Dependencies: {}
# [doc = " Returns an iterator over all occurrences of the needle in a haystack, in"] # [doc = " reverse."] # [inline] pub fn memrchr_iter (needle : u8 , haystack : & [u8]) -> Rev < Memchr < '_ > > { Memchr :: new (needle , haystack) . rev () }
};
}
