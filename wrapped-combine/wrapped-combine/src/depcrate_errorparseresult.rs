// Generated macro for ParseResult (enum)
macro_rules! Depcrate_errorParseResult {
() => {
// Module: crate::error
// Provides: {"ParseResult"}
// Dependencies: {}
# [doc = " A `Result` type which has the committed status flattened into the result."] # [doc = " Conversions to and from `std::result::Result` can be done using `result.into()` or"] # [doc = " `From::from(result)`"] # [derive (Clone , PartialEq , Debug , Copy)] pub enum ParseResult < T , E > { # [doc = " The parser has succeeded and has committed to this parse. If a parser after this fails,"] # [doc = " other parser alternatives will not be attempted (`CommitErr` will be returned)"] CommitOk (T) , # [doc = " The parser has succeeded and has not committed to this parse. If a parser after this fails,"] # [doc = " other parser alternatives will be attempted (`PeekErr` will be returned)"] PeekOk (T) , # [doc = " The parser failed other parse alternatives will not be attempted."] CommitErr (E) , # [doc = " The parser failed but other parse alternatives may be attempted."] PeekErr (Tracked < E >) , }
};
}
