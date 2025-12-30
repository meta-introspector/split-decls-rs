// Generated macro for impl_240 (impl)
macro_rules! Depcrate_datetimeimpl_240 {
() => {
// Module: crate::datetime
// Provides: {"impl_240"}
// Dependencies: {}
# [doc = " Accepts a relaxed form of RFC3339."] # [doc = " A space or a 'T' are accepted as the separator between the date and time"] # [doc = " parts."] # [doc = ""] # [doc = " All of these examples are equivalent:"] # [doc = " ```"] # [doc = " # use chrono::{DateTime, Local};"] # [doc = " \"2012-12-12T12:12:12Z\".parse::<DateTime<Local>>()?;"] # [doc = " \"2012-12-12 12:12:12Z\".parse::<DateTime<Local>>()?;"] # [doc = " \"2012-12-12 12:12:12+0000\".parse::<DateTime<Local>>()?;"] # [doc = " \"2012-12-12 12:12:12+00:00\".parse::<DateTime<Local>>()?;"] # [doc = " # Ok::<(), chrono::ParseError>(())"] # [doc = " ```"] # [cfg (feature = "clock")] impl str :: FromStr for DateTime < Local > { type Err = ParseError ; fn from_str (s : & str) -> ParseResult < DateTime < Local > > { s . parse :: < DateTime < FixedOffset > > () . map (| dt | dt . with_timezone (& Local)) } }
};
}
