// Generated macro for impl_122 (impl)
macro_rules! Depcrate_manifest_rust_versionimpl_122 {
() => {
// Module: crate::manifest::rust_version
// Provides: {"impl_122"}
// Dependencies: {}
impl std :: str :: FromStr for RustVersion { type Err = RustVersionError ; fn from_str (value : & str) -> Result < Self , Self :: Err > { let partial = value . parse :: < PartialVersion > () ; let partial = partial . map_err (RustVersionErrorKind :: PartialVersion) ? ; partial . try_into () } }
};
}
