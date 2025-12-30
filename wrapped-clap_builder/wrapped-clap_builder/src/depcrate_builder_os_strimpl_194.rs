// Generated macro for impl_194 (impl)
macro_rules! Depcrate_builder_os_strimpl_194 {
() => {
// Module: crate::builder::os_str
// Provides: {"impl_194"}
// Dependencies: {}
impl From < OsStr > for std :: path :: PathBuf { fn from (name : OsStr) -> Self { std :: ffi :: OsString :: from (name) . into () } }
};
}
