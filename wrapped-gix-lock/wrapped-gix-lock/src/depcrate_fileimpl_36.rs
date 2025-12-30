// Generated macro for impl_36 (impl)
macro_rules! Depcrate_fileimpl_36 {
() => {
// Module: crate::file
// Provides: {"impl_36"}
// Dependencies: {}
impl Marker { # [doc = " Return the path at which the lock file resides"] pub fn lock_path (& self) -> & Path { & self . lock_path } # [doc = " Return the path at which the locked resource resides"] pub fn resource_path (& self) -> PathBuf { strip_lock_suffix (& self . lock_path) } }
};
}
