// Generated macro for impl_191 (impl)
macro_rules! Depcrate_builder_os_strimpl_191 {
() => {
// Module: crate::builder::os_str
// Provides: {"impl_191"}
// Dependencies: {}
impl From < & '_ & 'static str > for OsStr { fn from (name : & '_ & 'static str) -> Self { Self :: from_static_ref ((* name) . as_ref ()) } }
};
}
