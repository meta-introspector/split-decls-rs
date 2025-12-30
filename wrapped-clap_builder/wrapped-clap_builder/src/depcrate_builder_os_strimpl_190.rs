// Generated macro for impl_190 (impl)
macro_rules! Depcrate_builder_os_strimpl_190 {
() => {
// Module: crate::builder::os_str
// Provides: {"impl_190"}
// Dependencies: {}
impl From < & 'static str > for OsStr { fn from (name : & 'static str) -> Self { Self :: from_static_ref (name . as_ref ()) } }
};
}
