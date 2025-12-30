// Generated macro for try_quote (function)
macro_rules! Depcrate_bytestry_quote {
() => {
// Module: crate::bytes
// Provides: {"try_quote"}
// Dependencies: {}
# [doc = " Given a single word, return a string suitable to encode it as a shell argument."] # [doc = ""] # [doc = " Uses default settings.  The only error that can be returned is [`QuoteError::Nul`]."] # [doc = ""] # [doc = " Equivalent to [`Quoter::new().quote(in_bytes)`](Quoter)."] # [doc = ""] # [doc = " (That configuration never returns `Err`, so this function does not panic.)"] # [doc = ""] # [doc = " The string equivalent is [shlex::try_quote]."] pub fn try_quote (in_bytes : & [u8]) -> Result < Cow < [u8] > , QuoteError > { Quoter :: new () . quote (in_bytes) }
};
}
