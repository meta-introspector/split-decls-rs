// Generated macro for memrchr2_iter (function)
macro_rules! Depcrate_memchrmemrchr2_iter {
() => {
// Module: crate::memchr
// Provides: {"memrchr2_iter"}
// Dependencies: {}
# [doc = " Returns an iterator over all occurrences of the needles in a haystack, in"] # [doc = " reverse."] # [inline] pub fn memrchr2_iter (needle1 : u8 , needle2 : u8 , haystack : & [u8] ,) -> Rev < Memchr2 < '_ > > { Memchr2 :: new (needle1 , needle2 , haystack) . rev () }
};
}
