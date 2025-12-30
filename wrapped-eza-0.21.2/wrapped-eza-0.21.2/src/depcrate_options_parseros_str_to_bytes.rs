// Generated macro for os_str_to_bytes (function)
macro_rules! Depcrate_options_parseros_str_to_bytes {
() => {
// Module: crate::options::parser
// Provides: {"os_str_to_bytes"}
// Dependencies: {}
# [cfg (windows)] fn os_str_to_bytes (s : & OsStr) -> & [u8] { return s . to_str () . unwrap () . as_bytes () ; }
};
}
