// Generated macro for impl_1301 (impl)
macro_rules! Depcrate_iterimpl_1301 {
() => {
// Module: crate::iter
// Provides: {"impl_1301"}
// Dependencies: {}
impl < 'data , I : 'data + ? Sized > IntoParallelRefIterator < 'data > for I where & 'data I : IntoParallelIterator , { type Iter = < & 'data I as IntoParallelIterator > :: Iter ; type Item = < & 'data I as IntoParallelIterator > :: Item ; fn par_iter (& 'data self) -> Self :: Iter { self . into_par_iter () } }
};
}
