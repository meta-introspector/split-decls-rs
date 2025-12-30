// Generated macro for ParsedOffsetKind (enum)
macro_rules! Depcrate_fmt_offsetParsedOffsetKind {
() => {
// Module: crate::fmt::offset
// Provides: {"ParsedOffsetKind"}
// Dependencies: {}
# [doc = " The kind of a parsed offset."] # [derive (Debug)] enum ParsedOffsetKind { # [doc = " The zulu offset, corresponding to UTC in a context where the offset for"] # [doc = " civil time is unknown or unavailable."] Zulu , # [doc = " The specific numeric offset."] Numeric (Numeric) , }
};
}
