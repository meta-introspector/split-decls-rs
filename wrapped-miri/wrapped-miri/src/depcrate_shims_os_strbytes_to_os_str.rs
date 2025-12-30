// Generated macro for bytes_to_os_str (function)
macro_rules! Depcrate_shims_os_strbytes_to_os_str {
() => {
// Module: crate::shims::os_str
// Provides: {"bytes_to_os_str"}
// Dependencies: {}
# [cfg (not (unix))] pub fn bytes_to_os_str < 'tcx > (bytes : & [u8]) -> InterpResult < 'tcx , & OsStr > { let s = std :: str :: from_utf8 (bytes) . map_err (| _ | err_unsup_format ! ("{:?} is not a valid utf-8 string" , bytes)) ? ; interp_ok (OsStr :: new (s)) }
};
}
