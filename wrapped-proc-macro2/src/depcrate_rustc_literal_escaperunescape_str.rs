// Generated macro for unescape_str (function)
macro_rules! Depcrate_rustc_literal_escaperunescape_str {
() => {
// Module: crate::rustc_literal_escaper
// Provides: {"unescape_str"}
// Dependencies: {}
# [doc = " Unescape a string literal"] # [doc = ""] # [doc = " Takes the contents of a string literal (without quotes)"] # [doc = " and produces a sequence of escaped characters or errors,"] # [doc = " which are returned by invoking `callback`."] pub fn unescape_str (src : & str , callback : impl FnMut (Range < usize > , Result < char , EscapeError >)) { str :: unescape (src , callback) }
};
}
