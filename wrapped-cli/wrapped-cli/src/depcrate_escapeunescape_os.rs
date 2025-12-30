// Generated macro for unescape_os (function)
macro_rules! Depcrate_escapeunescape_os {
() => {
// Module: crate::escape
// Provides: {"unescape_os"}
// Dependencies: {}
# [doc = " Unescapes an OS string."] # [doc = ""] # [doc = " This is like [`unescape`], but accepts an OS string."] # [doc = ""] # [doc = " Note that this first lossily decodes the given OS string as UTF-8. That"] # [doc = " is, an escaped string (the thing given) should be valid UTF-8."] pub fn unescape_os (string : & OsStr) -> Vec < u8 > { unescape (& string . to_string_lossy ()) }
};
}
