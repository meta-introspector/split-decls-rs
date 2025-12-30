// Generated macro for impl_94 (impl)
macro_rules! Depcrate_stackimpl_94 {
() => {
// Module: crate::stack
// Provides: {"impl_94"}
// Dependencies: {}
impl Drop for Func { fn drop (& mut self) { if ! self . data . is_null () { (self . drop) (self . data) ; } unsafe { * self . offset -= self . size } ; } }
};
}
