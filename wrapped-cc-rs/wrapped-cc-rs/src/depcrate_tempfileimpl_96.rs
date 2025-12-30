// Generated macro for impl_96 (impl)
macro_rules! Depcrate_tempfileimpl_96 {
() => {
// Module: crate::tempfile
// Provides: {"impl_96"}
// Dependencies: {}
impl Drop for NamedTempfile { fn drop (& mut self) { self . file . take () ; let _ = remove_file (& self . path) ; } }
};
}
