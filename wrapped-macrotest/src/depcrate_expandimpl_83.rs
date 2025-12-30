// Generated macro for impl_83 (impl)
macro_rules! Depcrate_expandimpl_83 {
() => {
// Module: crate::expand
// Provides: {"impl_83"}
// Dependencies: {}
# [doc = " This `Drop` implementation will clean up the temporary crates when expansion is finished."] # [doc = " This is to prevent pollution of the filesystem with dormant files."] impl Drop for Project { fn drop (& mut self) { if let Err (e) = fs :: remove_dir_all (& self . dir) { eprintln ! ("Failed to cleanup the directory `{}`: {}" , self . dir . to_string_lossy () , e) ; } } }
};
}
