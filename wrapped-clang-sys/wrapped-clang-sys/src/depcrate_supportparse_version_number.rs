// Generated macro for parse_version_number (function)
macro_rules! Depcrate_supportparse_version_number {
() => {
// Module: crate::support
// Provides: {"parse_version_number"}
// Dependencies: {}
# [doc = " Parses a version number if possible, ignoring trailing non-digit characters."] fn parse_version_number (number : & str) -> Option < c_int > { number . chars () . take_while (| c | c . is_ascii_digit ()) . collect :: < String > () . parse () . ok () }
};
}
