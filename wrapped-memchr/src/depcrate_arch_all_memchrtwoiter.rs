// Generated macro for TwoIter (struct)
macro_rules! Depcrate_arch_all_memchrTwoIter {
() => {
// Module: crate::arch::all::memchr
// Provides: {"TwoIter"}
// Dependencies: {}
# [doc = " An iterator over all occurrences of two possible bytes in a haystack."] # [doc = ""] # [doc = " This iterator implements `DoubleEndedIterator`, which means it can also be"] # [doc = " used to find occurrences in reverse order."] # [doc = ""] # [doc = " This iterator is created by the [`Two::iter`] method."] # [doc = ""] # [doc = " The lifetime parameters are as follows:"] # [doc = ""] # [doc = " * `'a` refers to the lifetime of the underlying [`Two`] searcher."] # [doc = " * `'h` refers to the lifetime of the haystack being searched."] # [derive (Clone , Debug)] pub struct TwoIter < 'a , 'h > { # [doc = " The underlying memchr searcher."] searcher : & 'a Two , # [doc = " Generic iterator implementation."] it : generic :: Iter < 'h > , }
};
}
