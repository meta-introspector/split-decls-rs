// Generated macro for impl_72 (impl)
macro_rules! Depcrate_stackimpl_72 {
() => {
// Module: crate::stack
// Provides: {"impl_72"}
// Dependencies: {}
impl ToNormalPathComponents for & str { fn to_normal_path_components (& self) -> impl Iterator < Item = Result < & OsStr , to_normal_path_components :: Error > > { self . split ('/') . filter_map (| c | bytes_component_to_os_str (c . as_bytes () , (* self) . into ())) } }
};
}
