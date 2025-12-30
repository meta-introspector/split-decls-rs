// Generated macro for impl_69 (impl)
macro_rules! Depcrate_stackimpl_69 {
() => {
// Module: crate::stack
// Provides: {"impl_69"}
// Dependencies: {}
impl ToNormalPathComponents for PathBuf { fn to_normal_path_components (& self) -> impl Iterator < Item = Result < & OsStr , to_normal_path_components :: Error > > { self . components () . map (| c | component_to_os_str (c , self)) } }
};
}
