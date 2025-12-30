// Generated macro for impl_496 (impl)
macro_rules! Depcrate_vector_rayonimpl_496 {
() => {
// Module: crate::vector::rayon
// Provides: {"impl_496"}
// Dependencies: {}
impl < 'a , A > IntoParallelRefIterator < 'a > for Vector < A > where A : Clone + Send + Sync + 'a , { type Item = & 'a A ; type Iter = ParIter < 'a , A > ; fn par_iter (& 'a self) -> Self :: Iter { ParIter { focus : self . focus () , } } }
};
}
