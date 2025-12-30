// Generated macro for from_bstr (function)
macro_rules! Depcrate_convertfrom_bstr {
() => {
// Module: crate::convert
// Provides: {"from_bstr"}
// Dependencies: {}
# [doc = " Similar to [`try_from_bstr()`], but **panics** if malformed surrogates are encountered on Windows."] pub fn from_bstr < 'a > (input : impl Into < Cow < 'a , BStr > >) -> Cow < 'a , Path > { try_from_bstr (input) . expect ("prefix path doesn't contain ill-formed UTF-8") }
};
}
