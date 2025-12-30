// Generated macro for impl_123 (impl)
macro_rules! Depcrate_manifest_rust_versionimpl_123 {
() => {
// Module: crate::manifest::rust_version
// Provides: {"impl_123"}
// Dependencies: {}
impl TryFrom < semver :: Version > for RustVersion { type Error = RustVersionError ; fn try_from (version : semver :: Version) -> Result < Self , Self :: Error > { let version = PartialVersion :: from (version) ; Self :: try_from (version) } }
};
}
