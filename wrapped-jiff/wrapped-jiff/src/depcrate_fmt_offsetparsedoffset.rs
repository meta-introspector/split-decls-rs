// Generated macro for ParsedOffset (struct)
macro_rules! Depcrate_fmt_offsetParsedOffset {
() => {
// Module: crate::fmt::offset
// Provides: {"ParsedOffset"}
// Dependencies: {}
# [doc = " An offset that has been parsed from a datetime string."] # [doc = ""] # [doc = " This represents either a Zulu offset (corresponding to UTC with an unknown"] # [doc = " time zone offset), or a specific numeric offset given in hours, minutes,"] # [doc = " seconds and nanoseconds (with everything except hours being optional)."] # [derive (Debug)] pub (crate) struct ParsedOffset { # [doc = " The kind of offset parsed."] kind : ParsedOffsetKind , }
};
}
