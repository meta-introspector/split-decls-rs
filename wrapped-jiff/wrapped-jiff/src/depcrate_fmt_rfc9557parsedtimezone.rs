// Generated macro for ParsedTimeZone (enum)
macro_rules! Depcrate_fmt_rfc9557ParsedTimeZone {
() => {
// Module: crate::fmt::rfc9557
// Provides: {"ParsedTimeZone"}
// Dependencies: {}
# [doc = " The result of parsing a time zone annotation."] # [derive (Debug)] enum ParsedTimeZone < 'i > { # [doc = " The name of an IANA time zone was found."] Named { # [doc = " Whether the critical flag was seen."] critical : bool , # [doc = " The parsed name."] name : & 'i str , } , # [doc = " A specific UTC numeric offset was found."] Offset { # [doc = " Whether the critical flag was seen."] critical : bool , # [doc = " The parsed UTC offset."] offset : ParsedOffset , } , }
};
}
