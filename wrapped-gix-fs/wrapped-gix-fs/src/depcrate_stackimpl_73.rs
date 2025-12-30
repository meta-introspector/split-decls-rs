// Generated macro for impl_73 (impl)
macro_rules! Depcrate_stackimpl_73 {
() => {
// Module: crate::stack
// Provides: {"impl_73"}
// Dependencies: {}
impl ToNormalPathComponents for & BString { fn to_normal_path_components (& self) -> impl Iterator < Item = Result < & OsStr , to_normal_path_components :: Error > > { self . split (| b | * b == b'/') . filter_map (| c | bytes_component_to_os_str (c , self . as_bstr ())) } }
};
}
