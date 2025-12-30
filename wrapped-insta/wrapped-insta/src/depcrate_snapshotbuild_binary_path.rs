// Generated macro for build_binary_path (function)
macro_rules! Depcrate_snapshotbuild_binary_path {
() => {
// Module: crate::snapshot
// Provides: {"build_binary_path"}
// Dependencies: {}
fn build_binary_path (extension : & str , path : impl Into < PathBuf >) -> PathBuf { let path = path . into () ; let mut new_extension = path . extension () . unwrap () . to_os_string () ; new_extension . push (".") ; new_extension . push (extension) ; path . with_extension (new_extension) }
};
}
