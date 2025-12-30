// Generated macro for impl_68 (impl)
macro_rules! Depcrate_stackimpl_68 {
() => {
// Module: crate::stack
// Provides: {"impl_68"}
// Dependencies: {}
impl ToNormalPathComponents for & Path { fn to_normal_path_components (& self) -> impl Iterator < Item = Result < & OsStr , to_normal_path_components :: Error > > { self . components () . map (| c | component_to_os_str (c , self)) } }
};
}
