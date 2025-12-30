// Generated macro for impl_792 (impl)
macro_rules! Depcrate_base_conversionimpl_792 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_792"}
// Dependencies: {}
impl < 'a , T : Scalar + Copy , R : Dim , C : Dim , S : RawStorage < T , R , C > + IsContiguous > From < & 'a Matrix < T , R , C , S > > for & 'a [T] { # [inline] fn from (matrix : & 'a Matrix < T , R , C , S >) -> Self { matrix . as_slice () } }
};
}
