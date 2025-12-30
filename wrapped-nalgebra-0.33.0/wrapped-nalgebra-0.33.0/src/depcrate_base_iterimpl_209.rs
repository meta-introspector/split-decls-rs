// Generated macro for impl_209 (impl)
macro_rules! Depcrate_base_iterimpl_209 {
() => {
// Module: crate::base::iter
// Provides: {"impl_209"}
// Dependencies: {}
impl < 'a , T , R : Dim , C : Dim , RStride : Dim , CStride : Dim > MatrixIter < 'a , T , R , C , ViewStorage < 'a , T , R , C , RStride , CStride > > { # [doc = " Creates a new iterator for the given matrix storage view."] pub fn new_owned (storage : ViewStorage < 'a , T , R , C , RStride , CStride >) -> Self { Self { inner : RawIter :: < * const T , T , R , C , RStride , CStride > :: new (& storage) , _marker : PhantomData , } } }
};
}
