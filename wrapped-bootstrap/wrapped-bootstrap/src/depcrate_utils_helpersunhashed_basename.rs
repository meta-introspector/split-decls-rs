// Generated macro for unhashed_basename (function)
macro_rules! Depcrate_utils_helpersunhashed_basename {
() => {
// Module: crate::utils::helpers
// Provides: {"unhashed_basename"}
// Dependencies: {}
# [doc = " Returns the filename without the hash prefix added by the cc crate."] # [doc = ""] # [doc = " Since v1.0.78 of the cc crate, object files are prefixed with a 16-character hash"] # [doc = " to avoid filename collisions."] pub fn unhashed_basename (obj : & Path) -> & str { let basename = obj . file_stem () . unwrap () . to_str () . expect ("UTF-8 file name") ; basename . split_once ('-') . unwrap () . 1 }
};
}
