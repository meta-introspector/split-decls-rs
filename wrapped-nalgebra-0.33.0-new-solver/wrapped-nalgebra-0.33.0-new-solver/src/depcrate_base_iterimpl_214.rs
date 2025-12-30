// Generated macro for impl_214 (impl)
macro_rules! Depcrate_base_iterimpl_214 {
() => {
// Module: crate::base::iter
// Provides: {"impl_214"}
// Dependencies: {}
impl < 'a , T : Scalar , R : Dim , C : Dim , S : 'a + RawStorage < T , R , C > > ExactSizeIterator for RowIter < 'a , T , R , C , S > { # [inline] fn len (& self) -> usize { self . mat . nrows () - self . curr } }
};
}
