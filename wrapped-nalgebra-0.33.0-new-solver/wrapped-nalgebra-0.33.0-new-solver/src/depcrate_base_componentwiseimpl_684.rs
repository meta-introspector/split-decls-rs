// Generated macro for impl_684 (impl)
macro_rules! Depcrate_base_componentwiseimpl_684 {
() => {
// Module: crate::base::componentwise
// Provides: {"impl_684"}
// Dependencies: {}
impl < T : Scalar , R : Dim , C : Dim , S : Storage < T , R , C > > Matrix < T , R , C , S > { # [doc = " Computes the component-wise absolute value."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nalgebra::Matrix2;"] # [doc = " let a = Matrix2::new(0.0, 1.0,"] # [doc = "                      -2.0, -3.0);"] # [doc = " assert_eq!(a.abs(), Matrix2::new(0.0, 1.0, 2.0, 3.0))"] # [doc = " ```"] # [inline] # [must_use] pub fn abs (& self) -> OMatrix < T , R , C > where T : Signed , DefaultAllocator : Allocator < R , C > , { let mut res = self . clone_owned () ; for e in res . iter_mut () { * e = e . abs () ; } res } }
};
}
