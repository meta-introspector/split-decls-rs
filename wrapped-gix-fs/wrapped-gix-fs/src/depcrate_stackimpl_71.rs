// Generated macro for impl_71 (impl)
macro_rules! Depcrate_stackimpl_71 {
() => {
// Module: crate::stack
// Provides: {"impl_71"}
// Dependencies: {}
impl ToNormalPathComponents for & BStr { fn to_normal_path_components (& self) -> impl Iterator < Item = Result < & OsStr , to_normal_path_components :: Error > > { self . split (| b | * b == b'/') . filter_map (| c | bytes_component_to_os_str (c , self)) } }
};
}
