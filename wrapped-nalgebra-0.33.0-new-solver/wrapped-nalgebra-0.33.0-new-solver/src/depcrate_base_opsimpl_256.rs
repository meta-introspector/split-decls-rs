// Generated macro for impl_256 (impl)
macro_rules! Depcrate_base_opsimpl_256 {
() => {
// Module: crate::base::ops
// Provides: {"impl_256"}
// Dependencies: {}
impl < T , C : Dim > iter :: Sum for OMatrix < T , Dyn , C > where T : Scalar + ClosedAddAssign + Zero , DefaultAllocator : Allocator < Dyn , C > , { # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::DVector;"] # [doc = " assert_eq!(vec![DVector::repeat(3, 1.0f64),"] # [doc = "                 DVector::repeat(3, 1.0f64),"] # [doc = "                 DVector::repeat(3, 1.0f64)].into_iter().sum::<DVector<f64>>(),"] # [doc = "            DVector::repeat(3, 1.0f64) + DVector::repeat(3, 1.0f64) + DVector::repeat(3, 1.0f64));"] # [doc = " ```"] # [doc = ""] # [doc = " # Panics"] # [doc = " Panics if the iterator is empty:"] # [doc = " ```should_panic"] # [doc = " # use std::iter;"] # [doc = " # use nalgebra::DMatrix;"] # [doc = " iter::empty::<DMatrix<f64>>().sum::<DMatrix<f64>>(); // panics!"] # [doc = " ```"] fn sum < I : Iterator < Item = OMatrix < T , Dyn , C > > > (mut iter : I) -> OMatrix < T , Dyn , C > { if let Some (first) = iter . next () { iter . fold (first , | acc , x | acc + x) } else { panic ! ("Cannot compute `sum` of empty iterator.") } } }
};
}
