// Generated macro for impl_519 (impl)
macro_rules! Depcrate_argimpl_519 {
() => {
// Module: crate::arg
// Provides: {"impl_519"}
// Dependencies: {}
impl < 'a > Iterator for Iter < 'a > { type Item = Box < dyn RefArg + 'static > ; fn next (& mut self) -> Option < Self :: Item > { let r = self . get_refarg () ; if r . is_some () { self . next () ; } r } }
};
}
