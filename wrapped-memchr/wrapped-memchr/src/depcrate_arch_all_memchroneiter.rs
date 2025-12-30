// Generated macro for OneIter (struct)
macro_rules! Depcrate_arch_all_memchrOneIter {
() => {
// Module: crate::arch::all::memchr
// Provides: {"OneIter"}
// Dependencies: {}
# [doc = " An iterator over all occurrences of a single byte in a haystack."] # [doc = ""] # [doc = " This iterator implements `DoubleEndedIterator`, which means it can also be"] # [doc = " used to find occurrences in reverse order."] # [doc = ""] # [doc = " This iterator is created by the [`One::iter`] method."] # [doc = ""] # [doc = " The lifetime parameters are as follows:"] # [doc = ""] # [doc = " * `'a` refers to the lifetime of the underlying [`One`] searcher."] # [doc = " * `'h` refers to the lifetime of the haystack being searched."] # [derive (Clone , Debug)] pub struct OneIter < 'a , 'h > { # [doc = " The underlying memchr searcher."] searcher : & 'a One , # [doc = " Generic iterator implementation."] it : generic :: Iter < 'h > , }
};
}
