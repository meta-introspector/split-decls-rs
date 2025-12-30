// Generated macro for Utf16CharMerger (struct)
macro_rules! Depcrate_decoding_iteratorsUtf16CharMerger {
() => {
// Module: crate::decoding_iterators
// Provides: {"Utf16CharMerger"}
// Dependencies: {}
# [doc = " Decodes UTF-16 characters from a `u16` iterator into `Utf16Char`s."] # [doc = ""] # [doc = " See [`IterExt::to_utf16chars()`](../trait.IterExt.html#tymethod.to_utf16chars)"] # [doc = " for examples and error handling."] # [derive (Clone , Default)] pub struct Utf16CharMerger < B : Borrow < u16 > , I : Iterator < Item = B > > { iter : I , # [doc = " Used when a trailing surrogate was expected, the u16 can be any value."] prev : Option < B > , }
};
}
