// Generated macro for impl_516 (impl)
macro_rules! Depcrate_repository_kindimpl_516 {
() => {
// Module: crate::repository::kind
// Provides: {"impl_516"}
// Dependencies: {}
impl Kind { # [doc = " Returns true if this is a bare repository, one without a work tree."] pub fn is_bare (& self) -> bool { matches ! (self , Kind :: Bare) } }
};
}
