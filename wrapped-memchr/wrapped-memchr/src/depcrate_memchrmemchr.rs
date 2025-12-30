// Generated macro for Memchr (struct)
macro_rules! Depcrate_memchrMemchr {
() => {
// Module: crate::memchr
// Provides: {"Memchr"}
// Dependencies: {}
# [doc = " An iterator over all occurrences of a single byte in a haystack."] # [doc = ""] # [doc = " This iterator implements `DoubleEndedIterator`, which means it can also be"] # [doc = " used to find occurrences in reverse order."] # [doc = ""] # [doc = " This iterator is created by the [`memchr_iter`] or `[memrchr_iter`]"] # [doc = " functions. It can also be created with the [`Memchr::new`] method."] # [doc = ""] # [doc = " The lifetime parameter `'h` refers to the lifetime of the haystack being"] # [doc = " searched."] # [derive (Clone , Debug)] pub struct Memchr < 'h > { needle1 : u8 , it : crate :: arch :: generic :: memchr :: Iter < 'h > , }
};
}
