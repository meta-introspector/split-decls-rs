// Generated macro for impl_404 (impl)
macro_rules! Depcrate_warningsimpl_404 {
() => {
// Module: crate::warnings
// Provides: {"impl_404"}
// Dependencies: {}
impl Warnings { # [doc = " Panics if any warnings are present."] # [track_caller] pub fn unwrap (& self) { if ! self . is_empty () { panic ! ("{self}") ; } } }
};
}
