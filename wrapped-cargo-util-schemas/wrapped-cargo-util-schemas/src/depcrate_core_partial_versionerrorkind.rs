// Generated macro for ErrorKind (enum)
macro_rules! Depcrate_core_partial_versionErrorKind {
() => {
// Module: crate::core::partial_version
// Provides: {"ErrorKind"}
// Dependencies: {}
# [doc = " Non-public error kind for [`PartialVersionError`]."] # [non_exhaustive] # [derive (Debug , thiserror :: Error)] enum ErrorKind { # [error ("unexpected version requirement, expected a version like \"1.32\"")] VersionReq , # [error ("unexpected prerelease field, expected a version like \"1.32\"")] Prerelease , # [error ("unexpected build field, expected a version like \"1.32\"")] BuildMetadata , # [error ("expected a version like \"1.32\"")] Unexpected , }
};
}
