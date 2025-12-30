// Generated macro for Parser (struct)
macro_rules! Depcrate_fmt_offsetParser {
() => {
// Module: crate::fmt::offset
// Provides: {"Parser"}
// Dependencies: {}
# [doc = " A parser for UTC offsets."] # [doc = ""] # [doc = " At time of writing, the typical configuration for offset parsing is to"] # [doc = " enable Zulu support and subminute precision. But when parsing zoned"] # [doc = " datetimes, and specifically, offsets within time zone annotations (the RFC"] # [doc = " 9557 extension to RFC 3339), then neither zulu nor subminute support are"] # [doc = " enabled."] # [doc = ""] # [doc = " N.B. I'm not actually totally clear on why zulu/subminute aren't allowed in"] # [doc = " time zone annotations, but that's what Temporal's grammar seems to dictate."] # [doc = " One might argue that this is what RFCs 3339 and 9557 require, but the"] # [doc = " Temporal grammar is already recognizing a superset anyway."] # [derive (Debug)] pub (crate) struct Parser { zulu : bool , require_minute : bool , require_second : bool , subminute : bool , subsecond : bool , colon : Colon , }
};
}
