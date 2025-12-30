// Generated macro for unescape_char (function)
macro_rules! Depcrate_rustc_literal_escaperunescape_char {
() => {
// Module: crate::rustc_literal_escaper
// Provides: {"unescape_char"}
// Dependencies: {}
# [doc = " Unescape a char literal"] # [doc = ""] # [doc = " Takes the contents of a char literal (without quotes),"] # [doc = " and returns an unescaped char or an error."] # [inline] pub fn unescape_char (src : & str) -> Result < char , EscapeError > { str :: unescape_single (& mut src . chars ()) }
};
}
