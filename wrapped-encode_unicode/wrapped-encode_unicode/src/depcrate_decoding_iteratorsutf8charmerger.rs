// Generated macro for Utf8CharMerger (struct)
macro_rules! Depcrate_decoding_iteratorsUtf8CharMerger {
() => {
// Module: crate::decoding_iterators
// Provides: {"Utf8CharMerger"}
// Dependencies: {}
# [doc = " Decodes UTF-8 characters from a byte iterator into `Utf8Char`s."] # [doc = ""] # [doc = " See [`IterExt::to_utf8chars()`](../trait.IterExt.html#tymethod.to_utf8chars)"] # [doc = " for examples and error handling."] # [derive (Clone , Default)] pub struct Utf8CharMerger < B : Borrow < u8 > , I : Iterator < Item = B > > { iter : I , # [doc = " number of bytes that were read before an error was detected"] after_err_leftover : u8 , # [doc = " stack because it simplifies popping."] after_err_stack : [u8 ; 3] , }
};
}
