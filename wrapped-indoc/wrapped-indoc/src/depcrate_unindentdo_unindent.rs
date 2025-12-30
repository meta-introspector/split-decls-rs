// Generated macro for do_unindent (function)
macro_rules! Depcrate_unindentdo_unindent {
() => {
// Module: crate::unindent
// Provides: {"do_unindent"}
// Dependencies: {}
pub (crate) fn do_unindent (s : & str , preserve_empty_first_line : bool) -> String { let bytes = s . as_bytes () ; let unindented = do_unindent_bytes (bytes , preserve_empty_first_line) ; String :: from_utf8 (unindented) . unwrap () }
};
}
