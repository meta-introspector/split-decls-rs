// Generated macro for parse_integer (function)
macro_rules! Depcrate_display_hintparse_integer {
() => {
// Module: crate::display_hint
// Provides: {"parse_integer"}
// Dependencies: {}
# [doc = " Parses an integer at the beginning of `s`."] # [doc = ""] # [doc = " Returns the integer and remaining text, if `s` started with an integer. Any errors parsing the"] # [doc = " number (which we already know only contains digits) are silently ignored."] fn parse_integer < T : FromStr > (s : & str) -> Option < (& str , T) > { let start_digits = s . as_bytes () . iter () . copied () . take_while (| b | b . is_ascii_digit ()) . count () ; let num = s [.. start_digits] . parse () . ok () ? ; Some ((& s [start_digits ..] , num)) }
};
}
