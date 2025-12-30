// Generated macro for Memchr3 (struct)
macro_rules! Depcrate_memchrMemchr3 {
() => {
// Module: crate::memchr
// Provides: {"Memchr3"}
// Dependencies: {}
# [doc = " An iterator over all occurrences of three possible bytes in a haystack."] # [doc = ""] # [doc = " This iterator implements `DoubleEndedIterator`, which means it can also be"] # [doc = " used to find occurrences in reverse order."] # [doc = ""] # [doc = " This iterator is created by the [`memchr2_iter`] or `[memrchr2_iter`]"] # [doc = " functions. It can also be created with the [`Memchr3::new`] method."] # [doc = ""] # [doc = " The lifetime parameter `'h` refers to the lifetime of the haystack being"] # [doc = " searched."] # [derive (Clone , Debug)] pub struct Memchr3 < 'h > { needle1 : u8 , needle2 : u8 , needle3 : u8 , it : crate :: arch :: generic :: memchr :: Iter < 'h > , }
};
}
