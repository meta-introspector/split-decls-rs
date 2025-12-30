// Generated macro for impl_224 (impl)
macro_rules! Depcrate_animationimpl_224 {
() => {
// Module: crate::animation
// Provides: {"impl_224"}
// Dependencies: {}
impl Clone for Frame { fn clone (& self) -> Self { Self { delay : self . delay , left : self . left , top : self . top , buffer : self . buffer . clone () , } } fn clone_from (& mut self , source : & Self) { self . delay = source . delay ; self . left = source . left ; self . top = source . top ; self . buffer . clone_from (& source . buffer) ; } }
};
}
