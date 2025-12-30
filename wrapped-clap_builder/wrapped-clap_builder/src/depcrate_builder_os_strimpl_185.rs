// Generated macro for impl_185 (impl)
macro_rules! Depcrate_builder_os_strimpl_185 {
() => {
// Module: crate::builder::os_str
// Provides: {"impl_185"}
// Dependencies: {}
# [cfg (feature = "string")] impl From < & '_ std :: ffi :: OsString > for OsStr { fn from (name : & '_ std :: ffi :: OsString) -> Self { Self :: from_ref (name . as_os_str ()) } }
};
}
