// Generated macro for is_tz_name_separator (function)
macro_rules! Depcrate_parsers_grammaris_tz_name_separator {
() => {
// Module: crate::parsers::grammar
// Provides: {"is_tz_name_separator"}
// Dependencies: {}
# [doc = " Checks if ascii char is a `TimeZoneIANAName` Separator."] # [inline] pub (crate) const fn is_tz_name_separator (ch : u8) -> bool { ch == b'/' }
};
}
