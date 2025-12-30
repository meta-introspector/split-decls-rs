// Generated macro for impl_338 (impl)
macro_rules! Depcrate_memchrimpl_338 {
() => {
// Module: crate::memchr
// Provides: {"impl_338"}
// Dependencies: {}
impl < 'h > Memchr < 'h > { # [doc = " Returns an iterator over all occurrences of the needle byte in the"] # [doc = " given haystack."] # [doc = ""] # [doc = " The iterator returned implements `DoubleEndedIterator`. This means it"] # [doc = " can also be used to find occurrences in reverse order."] # [inline] pub fn new (needle1 : u8 , haystack : & 'h [u8]) -> Memchr < 'h > { Memchr { needle1 , it : crate :: arch :: generic :: memchr :: Iter :: new (haystack) , } } }
};
}
