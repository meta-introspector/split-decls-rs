// Generated macro for bytes_to_os_str (function)
macro_rules! Depcrate_options_parserbytes_to_os_str {
() => {
// Module: crate::options::parser
// Provides: {"bytes_to_os_str"}
// Dependencies: {}
# [cfg (windows)] fn bytes_to_os_str (b : & [u8]) -> & OsStr { use std :: str ; return OsStr :: new (str :: from_utf8 (b) . unwrap ()) ; }
};
}
