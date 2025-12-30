// Generated macro for memrchr3_iter (function)
macro_rules! Depcrate_memchrmemrchr3_iter {
() => {
// Module: crate::memchr
// Provides: {"memrchr3_iter"}
// Dependencies: {}
# [doc = " Returns an iterator over all occurrences of the needles in a haystack, in"] # [doc = " reverse."] # [inline] pub fn memrchr3_iter (needle1 : u8 , needle2 : u8 , needle3 : u8 , haystack : & [u8] ,) -> Rev < Memchr3 < '_ > > { Memchr3 :: new (needle1 , needle2 , needle3 , haystack) . rev () }
};
}
