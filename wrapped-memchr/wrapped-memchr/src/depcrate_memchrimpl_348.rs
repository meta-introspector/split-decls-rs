// Generated macro for impl_348 (impl)
macro_rules! Depcrate_memchrimpl_348 {
() => {
// Module: crate::memchr
// Provides: {"impl_348"}
// Dependencies: {}
impl < 'h > Memchr3 < 'h > { # [doc = " Returns an iterator over all occurrences of the needle bytes in the"] # [doc = " given haystack."] # [doc = ""] # [doc = " The iterator returned implements `DoubleEndedIterator`. This means it"] # [doc = " can also be used to find occurrences in reverse order."] # [inline] pub fn new (needle1 : u8 , needle2 : u8 , needle3 : u8 , haystack : & 'h [u8] ,) -> Memchr3 < 'h > { Memchr3 { needle1 , needle2 , needle3 , it : crate :: arch :: generic :: memchr :: Iter :: new (haystack) , } } }
};
}
