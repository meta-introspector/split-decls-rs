// Generated macro for try_quote (function)
macro_rules! Depcratetry_quote {
() => {
// Module: crate
// Provides: {"try_quote"}
// Dependencies: {}
# [doc = " Given a single word, return a string suitable to encode it as a shell argument."] # [doc = ""] # [doc = " Uses default settings.  The only error that can be returned is [`QuoteError::Nul`]."] # [doc = ""] # [doc = " Equivalent to [`Quoter::new().quote(in_str)`](Quoter)."] # [doc = ""] # [doc = " (That configuration never returns `Err`, so this function does not panic.)"] # [doc = ""] # [doc = " The bytes equivalent is [bytes::try_quote]."] pub fn try_quote (in_str : & str) -> Result < Cow < str > , QuoteError > { Quoter :: new () . quote (in_str) }
};
}
