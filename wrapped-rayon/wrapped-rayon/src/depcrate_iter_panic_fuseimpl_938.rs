// Generated macro for impl_938 (impl)
macro_rules! Depcrate_iter_panic_fuseimpl_938 {
() => {
// Module: crate::iter::panic_fuse
// Provides: {"impl_938"}
// Dependencies: {}
impl < 'a , I > DoubleEndedIterator for PanicFuseIter < 'a , I > where I : DoubleEndedIterator , { fn next_back (& mut self) -> Option < Self :: Item > { if self . fuse . panicked () { None } else { self . base . next_back () } } }
};
}
