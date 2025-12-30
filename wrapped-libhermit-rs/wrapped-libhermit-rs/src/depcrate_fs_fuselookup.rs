// Generated macro for lookup (function)
macro_rules! Depcrate_fs_fuselookup {
() => {
// Module: crate::fs::fuse
// Provides: {"lookup"}
// Dependencies: {}
fn lookup (name : CString) -> Option < u64 > { let (cmd , rsp_payload_len) = ops :: Lookup :: create (name) ; let rsp = get_filesystem_driver () . unwrap () . lock () . send_command (cmd , rsp_payload_len) . ok () ? ; Some (rsp . headers . op_header . nodeid) }
};
}
