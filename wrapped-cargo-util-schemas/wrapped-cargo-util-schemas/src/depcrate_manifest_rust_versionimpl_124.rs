// Generated macro for impl_124 (impl)
macro_rules! Depcrate_manifest_rust_versionimpl_124 {
() => {
// Module: crate::manifest::rust_version
// Provides: {"impl_124"}
// Dependencies: {}
impl TryFrom < PartialVersion > for RustVersion { type Error = RustVersionError ; fn try_from (partial : PartialVersion) -> Result < Self , Self :: Error > { if partial . pre . is_some () { return Err (RustVersionErrorKind :: Prerelease . into ()) ; } if partial . build . is_some () { return Err (RustVersionErrorKind :: BuildMetadata . into ()) ; } Ok (Self (partial)) } }
};
}
