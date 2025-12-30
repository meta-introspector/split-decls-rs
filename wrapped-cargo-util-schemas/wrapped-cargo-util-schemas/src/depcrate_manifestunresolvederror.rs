// Generated macro for UnresolvedError (struct)
macro_rules! Depcrate_manifestUnresolvedError {
() => {
// Module: crate::manifest
// Provides: {"UnresolvedError"}
// Dependencies: {}
# [doc = " Error validating names in Cargo."] # [derive (Debug , thiserror :: Error)] # [error ("manifest field was not resolved")] # [non_exhaustive] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub struct UnresolvedError ;
};
}
