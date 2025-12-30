// Generated macro for impl_223 (impl)
macro_rules! Depcrate_base_iterimpl_223 {
() => {
// Module: crate::base::iter
// Provides: {"impl_223"}
// Dependencies: {}
impl < 'a , T : Scalar , R : Dim , C : Dim , S : 'a + RawStorage < T , R , C > > ExactSizeIterator for ColumnIter < 'a , T , R , C , S > { # [inline] fn len (& self) -> usize { self . range . end - self . range . start } }
};
}
