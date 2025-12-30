// Generated macro for char_to_escaped_literal (function)
macro_rules! Depcrate_globchar_to_escaped_literal {
() => {
// Module: crate::glob
// Provides: {"char_to_escaped_literal"}
// Dependencies: {}
# [doc = " Convert a Unicode scalar value to an escaped string suitable for use as"] # [doc = " a literal in a non-Unicode regex."] fn char_to_escaped_literal (c : char) -> String { let mut buf = [0 ; 4] ; let bytes = c . encode_utf8 (& mut buf) . as_bytes () ; bytes_to_escaped_literal (bytes) }
};
}
