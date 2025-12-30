// Generated macro for impl_181 (impl)
macro_rules! Depcrate_builder_os_strimpl_181 {
() => {
// Module: crate::builder::os_str
// Provides: {"impl_181"}
// Dependencies: {}
# [cfg (feature = "string")] impl From < Str > for OsStr { fn from (id : Str) -> Self { match id . into_inner () { crate :: builder :: StrInner :: Static (s) => Self :: from_static_ref (std :: ffi :: OsStr :: new (s)) , crate :: builder :: StrInner :: Owned (s) => Self :: from_ref (std :: ffi :: OsStr :: new (s . as_ref ())) , } } }
};
}
