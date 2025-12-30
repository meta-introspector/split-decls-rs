// Generated macro for colon_or_space (function)
macro_rules! Depcrate_format_scancolon_or_space {
() => {
// Module: crate::format::scan
// Provides: {"colon_or_space"}
// Dependencies: {}
# [doc = " Consumes any number (including zero) of colon or spaces."] pub (crate) fn colon_or_space (s : & str) -> ParseResult < & str > { Ok (s . trim_start_matches (| c : char | c == ':' || c . is_whitespace ())) }
};
}
