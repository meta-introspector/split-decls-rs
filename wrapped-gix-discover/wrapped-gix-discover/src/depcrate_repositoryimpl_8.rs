// Generated macro for impl_8 (impl)
macro_rules! Depcrate_repositoryimpl_8 {
() => {
// Module: crate::repository
// Provides: {"impl_8"}
// Dependencies: {}
impl Kind { # [doc = " Returns true if this is a bare repository, one without a work tree."] pub fn is_bare (& self) -> bool { matches ! (self , Kind :: PossiblyBare) } }
};
}
