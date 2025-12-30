// Generated macro for impl_60 (impl)
macro_rules! Depcrate_versionimpl_60 {
() => {
// Module: crate::version
// Provides: {"impl_60"}
// Dependencies: {}
impl Version { # [doc = " Is this version expected to have a public key?"] pub fn has_public_key (self) -> bool { match self { Version :: V1 => false , Version :: V2 => true , } } }
};
}
