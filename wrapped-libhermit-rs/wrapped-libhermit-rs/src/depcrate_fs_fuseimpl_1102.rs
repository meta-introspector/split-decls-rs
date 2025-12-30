// Generated macro for impl_1102 (impl)
macro_rules! Depcrate_fs_fuseimpl_1102 {
() => {
// Module: crate::fs::fuse
// Provides: {"impl_1102"}
// Dependencies: {}
impl Drop for FuseFileHandleInner { fn drop (& mut self) { if let Some (fuse_nid) = self . fuse_nid && let Some (fuse_fh) = self . fuse_fh { let (cmd , rsp_payload_len) = ops :: Release :: create (fuse_nid , fuse_fh) ; get_filesystem_driver () . unwrap () . lock () . send_command (cmd , rsp_payload_len) . unwrap () ; } } }
};
}
