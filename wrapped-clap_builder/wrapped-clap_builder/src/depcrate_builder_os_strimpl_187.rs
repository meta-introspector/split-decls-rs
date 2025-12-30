// Generated macro for impl_187 (impl)
macro_rules! Depcrate_builder_os_strimpl_187 {
() => {
// Module: crate::builder::os_str
// Provides: {"impl_187"}
// Dependencies: {}
# [cfg (feature = "string")] impl From < & '_ String > for OsStr { fn from (name : & '_ String) -> Self { Self :: from_ref (name . as_str () . as_ref ()) } }
};
}
