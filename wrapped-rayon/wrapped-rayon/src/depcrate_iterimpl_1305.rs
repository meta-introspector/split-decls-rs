// Generated macro for impl_1305 (impl)
macro_rules! Depcrate_iterimpl_1305 {
() => {
// Module: crate::iter
// Provides: {"impl_1305"}
// Dependencies: {}
impl < T : ParallelIterator > IntoParallelIterator for T { type Iter = T ; type Item = T :: Item ; fn into_par_iter (self) -> T { self } }
};
}
