// Generated macro for impl_1303 (impl)
macro_rules! Depcrate_iterimpl_1303 {
() => {
// Module: crate::iter
// Provides: {"impl_1303"}
// Dependencies: {}
impl < 'data , I : 'data + ? Sized > IntoParallelRefMutIterator < 'data > for I where & 'data mut I : IntoParallelIterator , { type Iter = < & 'data mut I as IntoParallelIterator > :: Iter ; type Item = < & 'data mut I as IntoParallelIterator > :: Item ; fn par_iter_mut (& 'data mut self) -> Self :: Iter { self . into_par_iter () } }
};
}
