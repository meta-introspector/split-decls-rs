// Generated macro for RustVersionErrorKind (enum)
macro_rules! Depcrate_manifest_rust_versionRustVersionErrorKind {
() => {
// Module: crate::manifest::rust_version
// Provides: {"RustVersionErrorKind"}
// Dependencies: {}
# [doc = " Non-public error kind for [`RustVersionError`]."] # [non_exhaustive] # [derive (Debug , thiserror :: Error)] enum RustVersionErrorKind { # [error ("unexpected prerelease field, expected a version like \"1.32\"")] Prerelease , # [error ("unexpected build field, expected a version like \"1.32\"")] BuildMetadata , # [error (transparent)] PartialVersion (# [from] PartialVersionError) , }
};
}
