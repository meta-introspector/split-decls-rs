// Generated macro for try_join (function)
macro_rules! Depcratetry_join {
() => {
// Module: crate
// Provides: {"try_join"}
// Dependencies: {}
# [doc = " Convenience function that consumes an iterable of words and turns it into a single string,"] # [doc = " quoting words when necessary. Consecutive words will be separated by a single space."] # [doc = ""] # [doc = " Uses default settings.  The only error that can be returned is [`QuoteError::Nul`]."] # [doc = ""] # [doc = " Equivalent to [`Quoter::new().join(words)`](Quoter)."] # [doc = ""] # [doc = " The bytes equivalent is [bytes::try_join]."] pub fn try_join < 'a , I : IntoIterator < Item = & 'a str > > (words : I) -> Result < String , QuoteError > { Quoter :: new () . join (words) }
};
}
