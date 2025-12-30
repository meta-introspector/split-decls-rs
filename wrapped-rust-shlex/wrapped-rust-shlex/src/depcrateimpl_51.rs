// Generated macro for impl_51 (impl)
macro_rules! Depcrateimpl_51 {
() => {
// Module: crate
// Provides: {"impl_51"}
// Dependencies: {}
impl Quoter { # [doc = " Create a new [`Quoter`] with default settings."] # [inline] pub fn new () -> Self { Self :: default () } # [doc = " Set whether to allow [nul bytes](quoting_warning#nul-bytes).  By default they are not"] # [doc = " allowed and will result in an error of [`QuoteError::Nul`]."] # [inline] pub fn allow_nul (mut self , allow : bool) -> Self { self . inner = self . inner . allow_nul (allow) ; self } # [doc = " Convenience function that consumes an iterable of words and turns it into a single string,"] # [doc = " quoting words when necessary. Consecutive words will be separated by a single space."] pub fn join < 'a , I : IntoIterator < Item = & 'a str > > (& self , words : I) -> Result < String , QuoteError > { self . inner . join (words . into_iter () . map (| s | s . as_bytes ())) . map (| bytes | unsafe { String :: from_utf8_unchecked (bytes) }) } # [doc = " Given a single word, return a string suitable to encode it as a shell argument."] pub fn quote < 'a > (& self , in_str : & 'a str) -> Result < Cow < 'a , str > , QuoteError > { Ok (match self . inner . quote (in_str . as_bytes ()) ? { Cow :: Borrowed (out) => { unsafe { core :: str :: from_utf8_unchecked (out) } . into () } Cow :: Owned (out) => { unsafe { String :: from_utf8_unchecked (out) } . into () } }) } }
};
}
