// Generated macro for impl_239 (impl)
macro_rules! Depcrate_datetimeimpl_239 {
() => {
// Module: crate::datetime
// Provides: {"impl_239"}
// Dependencies: {}
# [doc = " Accepts a relaxed form of RFC3339."] # [doc = " A space or a 'T' are accepted as the separator between the date and time"] # [doc = " parts."] # [doc = ""] # [doc = " All of these examples are equivalent:"] # [doc = " ```"] # [doc = " # use chrono::{DateTime, Utc};"] # [doc = " \"2012-12-12T12:12:12Z\".parse::<DateTime<Utc>>()?;"] # [doc = " \"2012-12-12 12:12:12Z\".parse::<DateTime<Utc>>()?;"] # [doc = " \"2012-12-12 12:12:12+0000\".parse::<DateTime<Utc>>()?;"] # [doc = " \"2012-12-12 12:12:12+00:00\".parse::<DateTime<Utc>>()?;"] # [doc = " # Ok::<(), chrono::ParseError>(())"] # [doc = " ```"] impl str :: FromStr for DateTime < Utc > { type Err = ParseError ; fn from_str (s : & str) -> ParseResult < DateTime < Utc > > { s . parse :: < DateTime < FixedOffset > > () . map (| dt | dt . with_timezone (& Utc)) } }
};
}
