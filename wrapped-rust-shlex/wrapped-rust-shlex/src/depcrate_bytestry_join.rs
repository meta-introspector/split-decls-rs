// Generated macro for try_join (function)
macro_rules! Depcrate_bytestry_join {
() => {
// Module: crate::bytes
// Provides: {"try_join"}
// Dependencies: {}
# [doc = " Convenience function that consumes an iterable of words and turns it into a single byte string,"] # [doc = " quoting words when necessary. Consecutive words will be separated by a single space."] # [doc = ""] # [doc = " Uses default settings.  The only error that can be returned is [`QuoteError::Nul`]."] # [doc = ""] # [doc = " Equivalent to [`Quoter::new().join(words)`](Quoter)."] # [doc = ""] # [doc = " The string equivalent is [shlex::try_join]."] pub fn try_join < 'a , I : IntoIterator < Item = & 'a [u8] > > (words : I) -> Result < Vec < u8 > , QuoteError > { Quoter :: new () . join (words) }
};
}
