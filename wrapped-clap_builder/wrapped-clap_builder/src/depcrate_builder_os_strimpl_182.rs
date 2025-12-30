// Generated macro for impl_182 (impl)
macro_rules! Depcrate_builder_os_strimpl_182 {
() => {
// Module: crate::builder::os_str
// Provides: {"impl_182"}
// Dependencies: {}
# [cfg (not (feature = "string"))] impl From < Str > for OsStr { fn from (id : Str) -> Self { Self :: from_static_ref (std :: ffi :: OsStr :: new (id . into_inner () . 0)) } }
};
}
