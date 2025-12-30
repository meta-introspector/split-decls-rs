// Generated macro for to_native_separators (function)
macro_rules! Depcrate_convertto_native_separators {
() => {
// Module: crate::convert
// Provides: {"to_native_separators"}
// Dependencies: {}
# [doc = " Assures the given bytes use the native path separator."] pub fn to_native_separators < 'a > (path : impl Into < Cow < 'a , BStr > >) -> Cow < 'a , BStr > { # [cfg (not (windows))] let p = to_unix_separators (path) ; # [cfg (windows)] let p = to_windows_separators (path) ; p }
};
}
