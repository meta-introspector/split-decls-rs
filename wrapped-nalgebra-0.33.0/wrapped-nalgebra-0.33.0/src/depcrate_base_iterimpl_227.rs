// Generated macro for impl_227 (impl)
macro_rules! Depcrate_base_iterimpl_227 {
() => {
// Module: crate::base::iter
// Provides: {"impl_227"}
// Dependencies: {}
impl < 'a , T : Scalar , R : Dim , C : Dim , S : 'a + RawStorageMut < T , R , C > > ExactSizeIterator for ColumnIterMut < 'a , T , R , C , S > { # [inline] fn len (& self) -> usize { self . range . len () } }
};
}
