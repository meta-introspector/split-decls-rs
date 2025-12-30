// Generated macro for PartialVersionError (struct)
macro_rules! Depcrate_core_partial_versionPartialVersionError {
() => {
// Module: crate::core::partial_version
// Provides: {"PartialVersionError"}
// Dependencies: {}
# [doc = " Error parsing a [`PartialVersion`]."] # [derive (Debug , thiserror :: Error)] # [error (transparent)] pub struct PartialVersionError (# [from] ErrorKind) ;
};
}
