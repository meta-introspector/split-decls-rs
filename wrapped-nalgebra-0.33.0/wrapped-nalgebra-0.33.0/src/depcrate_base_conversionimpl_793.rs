// Generated macro for impl_793 (impl)
macro_rules! Depcrate_base_conversionimpl_793 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_793"}
// Dependencies: {}
impl < 'a , T : Scalar + Copy , R : Dim , C : Dim , S : RawStorageMut < T , R , C > + IsContiguous > From < & 'a mut Matrix < T , R , C , S > > for & 'a mut [T] { # [inline] fn from (matrix : & 'a mut Matrix < T , R , C , S >) -> Self { matrix . as_mut_slice () } }
};
}
