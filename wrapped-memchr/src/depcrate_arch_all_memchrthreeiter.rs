// Generated macro for ThreeIter (struct)
macro_rules! Depcrate_arch_all_memchrThreeIter {
() => {
// Module: crate::arch::all::memchr
// Provides: {"ThreeIter"}
// Dependencies: {}
# [doc = " An iterator over all occurrences of three possible bytes in a haystack."] # [doc = ""] # [doc = " This iterator implements `DoubleEndedIterator`, which means it can also be"] # [doc = " used to find occurrences in reverse order."] # [doc = ""] # [doc = " This iterator is created by the [`Three::iter`] method."] # [doc = ""] # [doc = " The lifetime parameters are as follows:"] # [doc = ""] # [doc = " * `'a` refers to the lifetime of the underlying [`Three`] searcher."] # [doc = " * `'h` refers to the lifetime of the haystack being searched."] # [derive (Clone , Debug)] pub struct ThreeIter < 'a , 'h > { # [doc = " The underlying memchr searcher."] searcher : & 'a Three , # [doc = " Generic iterator implementation."] it : generic :: Iter < 'h > , }
};
}
