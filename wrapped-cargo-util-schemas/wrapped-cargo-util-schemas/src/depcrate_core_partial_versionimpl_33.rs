// Generated macro for impl_33 (impl)
macro_rules! Depcrate_core_partial_versionimpl_33 {
() => {
// Module: crate::core::partial_version
// Provides: {"impl_33"}
// Dependencies: {}
impl From < semver :: Version > for PartialVersion { fn from (ver : semver :: Version) -> Self { let pre = if ver . pre . is_empty () { None } else { Some (ver . pre) } ; let build = if ver . build . is_empty () { None } else { Some (ver . build) } ; Self { major : ver . major , minor : Some (ver . minor) , patch : Some (ver . patch) , pre , build , } } }
};
}
