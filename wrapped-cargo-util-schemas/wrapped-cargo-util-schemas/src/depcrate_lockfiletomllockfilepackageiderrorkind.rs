// Generated macro for TomlLockfilePackageIdErrorKind (enum)
macro_rules! Depcrate_lockfileTomlLockfilePackageIdErrorKind {
() => {
// Module: crate::lockfile
// Provides: {"TomlLockfilePackageIdErrorKind"}
// Dependencies: {}
# [non_exhaustive] # [derive (Debug , thiserror :: Error)] enum TomlLockfilePackageIdErrorKind { # [error ("invalid serialied PackageId")] InvalidSerializedPackageId , # [error (transparent)] Source (# [from] TomlLockfileSourceIdError) , }
};
}
