// Generated macro for normalize_path (function)
macro_rules! Depcrate_pathutilnormalize_path {
() => {
// Module: crate::pathutil
// Provides: {"normalize_path"}
// Dependencies: {}
# [doc = " Normalizes a path to use `/` as a separator everywhere, even on platforms"] # [doc = " that recognize other characters as separators."] # [cfg (not (unix))] pub (crate) fn normalize_path (mut path : Cow < [u8] >) -> Cow < [u8] > { use std :: path :: is_separator ; for i in 0 .. path . len () { if path [i] == b'/' || ! is_separator (char :: from (path [i])) { continue ; } path . to_mut () [i] = b'/' ; } path }
};
}
