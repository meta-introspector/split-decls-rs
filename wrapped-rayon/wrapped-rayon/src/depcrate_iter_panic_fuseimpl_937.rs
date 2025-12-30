// Generated macro for impl_937 (impl)
macro_rules! Depcrate_iter_panic_fuseimpl_937 {
() => {
// Module: crate::iter::panic_fuse
// Provides: {"impl_937"}
// Dependencies: {}
impl < 'a , I > Iterator for PanicFuseIter < 'a , I > where I : Iterator , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { if self . fuse . panicked () { None } else { self . base . next () } } fn size_hint (& self) -> (usize , Option < usize >) { self . base . size_hint () } }
};
}
