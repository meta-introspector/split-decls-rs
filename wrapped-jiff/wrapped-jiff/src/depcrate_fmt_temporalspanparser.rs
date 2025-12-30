// Generated macro for SpanParser (struct)
macro_rules! Depcrate_fmt_temporalSpanParser {
() => {
// Module: crate::fmt::temporal
// Provides: {"SpanParser"}
// Dependencies: {}
# [doc = " A parser for Temporal durations."] # [doc = ""] # [doc = " Note that in Jiff, a \"Temporal duration\" is called a \"span.\""] # [doc = ""] # [doc = " See the [`fmt::temporal`](crate::fmt::temporal) module documentation for"] # [doc = " more information on the specific format used."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This example shows how to parse a [`Span`] from a byte string. (That is,"] # [doc = " `&[u8]` and not a `&str`.)"] # [doc = ""] # [doc = " ```"] # [doc = " use jiff::{fmt::temporal::SpanParser, ToSpan};"] # [doc = ""] # [doc = " // A parser can be created in a const context."] # [doc = " static PARSER: SpanParser = SpanParser::new();"] # [doc = ""] # [doc = " let span = PARSER.parse_span(b\"P3y7m25dT7h36m\")?;"] # [doc = " assert_eq!("] # [doc = "     span,"] # [doc = "     3.years().months(7).days(25).hours(7).minutes(36).fieldwise(),"] # [doc = " );"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] # [derive (Debug)] pub struct SpanParser { p : parser :: SpanParser , }
};
}
