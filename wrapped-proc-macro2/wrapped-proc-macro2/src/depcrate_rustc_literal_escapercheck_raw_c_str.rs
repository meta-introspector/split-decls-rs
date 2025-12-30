// Generated macro for check_raw_c_str (function)
macro_rules! Depcrate_rustc_literal_escapercheck_raw_c_str {
() => {
// Module: crate::rustc_literal_escaper
// Provides: {"check_raw_c_str"}
// Dependencies: {}
# [doc = " Check a raw C string literal for validity"] # [doc = ""] # [doc = " Takes the contents of a raw C string literal (without quotes)"] # [doc = " and produces a sequence of characters or errors,"] # [doc = " which are returned by invoking `callback`."] # [doc = " NOTE: Does no escaping, but produces errors for bare carriage return ('\\r')."] pub fn check_raw_c_str (src : & str , callback : impl FnMut (Range < usize > , Result < NonZeroChar , EscapeError >) ,) { CStr :: check_raw (src , callback) ; }
};
}
