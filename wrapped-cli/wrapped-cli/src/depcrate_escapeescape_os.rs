// Generated macro for escape_os (function)
macro_rules! Depcrate_escapeescape_os {
() => {
// Module: crate::escape
// Provides: {"escape_os"}
// Dependencies: {}
# [doc = " Escapes an OS string into a human readable string."] # [doc = ""] # [doc = " This is like [`escape`], but accepts an OS string."] pub fn escape_os (string : & OsStr) -> String { escape (Vec :: from_os_str_lossy (string) . as_bytes ()) }
};
}
