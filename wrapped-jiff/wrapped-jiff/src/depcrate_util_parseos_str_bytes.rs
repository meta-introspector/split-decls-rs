// Generated macro for os_str_bytes (function)
macro_rules! Depcrate_util_parseos_str_bytes {
() => {
// Module: crate::util::parse
// Provides: {"os_str_bytes"}
// Dependencies: {}
# [doc = " Parses an `OsStr` into a `&str` when `&[u8]` isn't easily available."] # [doc = ""] # [doc = " The main difference between this and `OsStr::to_str` is that this will"] # [doc = " be a zero-cost conversion on Unix platforms to `&[u8]`. On Windows, this"] # [doc = " will do UTF-8 validation and return an error if it's invalid UTF-8."] # [cfg (feature = "tz-system")] pub (crate) fn os_str_bytes < 'o , O > (os_str : & 'o O) -> Result < & 'o [u8] , Error > where O : ? Sized + AsRef < std :: ffi :: OsStr > , { let os_str = os_str . as_ref () ; # [cfg (unix)] { use std :: os :: unix :: ffi :: OsStrExt ; Ok (os_str . as_bytes ()) } # [cfg (not (unix))] { let string = os_str . to_str () . ok_or_else (| | { err ! ("environment value {os_str:?} is not valid UTF-8") }) ? ; Ok (string . as_bytes ()) } }
};
}
