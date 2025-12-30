// Generated macro for to_unix_separators_on_windows (function)
macro_rules! Depcrate_convertto_unix_separators_on_windows {
() => {
// Module: crate::convert
// Provides: {"to_unix_separators_on_windows"}
// Dependencies: {}
# [doc = " Replace Windows path separators with slashes, but only do so on Windows."] pub fn to_unix_separators_on_windows < 'a > (path : impl Into < Cow < 'a , BStr > >) -> Cow < 'a , BStr > { # [cfg (windows)] { to_unix_separators (path) } # [cfg (not (windows))] { path . into () } }
};
}
