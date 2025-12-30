// Generated macro for impl_497 (impl)
macro_rules! Depcrate_vector_rayonimpl_497 {
() => {
// Module: crate::vector::rayon
// Provides: {"impl_497"}
// Dependencies: {}
impl < 'a , A > IntoParallelRefMutIterator < 'a > for Vector < A > where A : Clone + Send + Sync + 'a , { type Item = & 'a mut A ; type Iter = ParIterMut < 'a , A > ; fn par_iter_mut (& 'a mut self) -> Self :: Iter { ParIterMut { focus : self . focus_mut () , } } }
};
}
