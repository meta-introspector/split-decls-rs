// Generated macro for impl_218 (impl)
macro_rules! Depcrate_base_iterimpl_218 {
() => {
// Module: crate::base::iter
// Provides: {"impl_218"}
// Dependencies: {}
impl < 'a , T : Scalar , R : Dim , C : Dim , S : 'a + RawStorageMut < T , R , C > > ExactSizeIterator for RowIterMut < 'a , T , R , C , S > { # [inline] fn len (& self) -> usize { self . nrows () - self . curr } }
};
}
