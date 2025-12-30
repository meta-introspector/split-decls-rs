// Generated macro for RustVersionError (struct)
macro_rules! Depcrate_manifest_rust_versionRustVersionError {
() => {
// Module: crate::manifest::rust_version
// Provides: {"RustVersionError"}
// Dependencies: {}
# [doc = " Error parsing a [`RustVersion`]."] # [derive (Debug , thiserror :: Error)] # [error (transparent)] pub struct RustVersionError (# [from] RustVersionErrorKind) ;
};
}
