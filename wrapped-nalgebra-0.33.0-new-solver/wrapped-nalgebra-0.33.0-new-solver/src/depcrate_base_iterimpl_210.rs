// Generated macro for impl_210 (impl)
macro_rules! Depcrate_base_iterimpl_210 {
() => {
// Module: crate::base::iter
// Provides: {"impl_210"}
// Dependencies: {}
impl < 'a , T , R : Dim , C : Dim , RStride : Dim , CStride : Dim > MatrixIterMut < 'a , T , R , C , ViewStorageMut < 'a , T , R , C , RStride , CStride > > { # [doc = " Creates a new iterator for the given matrix storage view."] pub fn new_owned_mut (mut storage : ViewStorageMut < 'a , T , R , C , RStride , CStride >) -> Self { Self { inner : RawIter :: < * mut T , T , R , C , RStride , CStride > :: new (& mut storage) , _marker : PhantomData , } } }
};
}
