// Generated macro for impl_258 (impl)
macro_rules! Depcrate_base_opsimpl_258 {
() => {
// Module: crate::base::ops
// Provides: {"impl_258"}
// Dependencies: {}
impl < 'a , T , C : Dim > iter :: Sum < & 'a OMatrix < T , Dyn , C > > for OMatrix < T , Dyn , C > where T : Scalar + ClosedAddAssign + Zero , DefaultAllocator : Allocator < Dyn , C > , { # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::DVector;"] # [doc = " let v = &DVector::repeat(3, 1.0f64);"] # [doc = ""] # [doc = " assert_eq!(vec![v, v, v].into_iter().sum::<DVector<f64>>(),"] # [doc = "            v + v + v);"] # [doc = " ```"] # [doc = ""] # [doc = " # Panics"] # [doc = " Panics if the iterator is empty:"] # [doc = " ```should_panic"] # [doc = " # use std::iter;"] # [doc = " # use nalgebra::DMatrix;"] # [doc = " iter::empty::<&DMatrix<f64>>().sum::<DMatrix<f64>>(); // panics!"] # [doc = " ```"] fn sum < I : Iterator < Item = & 'a OMatrix < T , Dyn , C > > > (mut iter : I) -> OMatrix < T , Dyn , C > { if let Some (first) = iter . next () { iter . fold (first . clone () , | acc , x | acc + x) } else { panic ! ("Cannot compute `sum` of empty iterator.") } } }
};
}
