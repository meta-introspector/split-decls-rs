// Generated macro for ParsedTime (struct)
macro_rules! Depcrate_fmt_temporal_parserParsedTime {
() => {
// Module: crate::fmt::temporal::parser
// Provides: {"ParsedTime"}
// Dependencies: {}
# [doc = " The result of parsing a 24-hour civil time."] # [derive (Debug)] pub (super) struct ParsedTime < 'i > { # [doc = " The original input that the time was parsed from."] input : escape :: Bytes < 'i > , # [doc = " The actual parsed time."] time : Time , # [doc = " Whether the time was parsed in extended format or not."] extended : bool , }
};
}
