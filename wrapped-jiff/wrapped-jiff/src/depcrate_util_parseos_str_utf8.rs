// Generated macro for os_str_utf8 (function)
macro_rules! Depcrate_util_parseos_str_utf8 {
() => {
// Module: crate::util::parse
// Provides: {"os_str_utf8"}
// Dependencies: {}
# [doc = " Parses an `OsStr` into a `&str` when `&[u8]` isn't easily available."] # [doc = ""] # [doc = " This is effectively `OsStr::to_str`, but with a slightly better error"] # [doc = " message."] # [cfg (feature = "tzdb-zoneinfo")] pub (crate) fn os_str_utf8 < 'o , O > (os_str : & 'o O) -> Result < & 'o str , Error > where O : ? Sized + AsRef < std :: ffi :: OsStr > , { let os_str = os_str . as_ref () ; os_str . to_str () . ok_or_else (| | err ! ("environment value {os_str:?} is not valid UTF-8")) }
};
}
