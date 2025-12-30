// Generated macro for osstr_as_utf8_bytes (function)
macro_rules! Depcrate_debuginfo_line_infoosstr_as_utf8_bytes {
() => {
// Module: crate::debuginfo::line_info
// Provides: {"osstr_as_utf8_bytes"}
// Dependencies: {}
fn osstr_as_utf8_bytes (path : & OsStr) -> & [u8] { # [cfg (unix)] { use std :: os :: unix :: ffi :: OsStrExt ; path . as_bytes () } # [cfg (not (unix))] { path . to_str () . unwrap () . as_bytes () } }
};
}
