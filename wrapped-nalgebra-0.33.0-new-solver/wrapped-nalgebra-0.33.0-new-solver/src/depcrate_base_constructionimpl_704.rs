// Generated macro for impl_704 (impl)
macro_rules! Depcrate_base_constructionimpl_704 {
() => {
// Module: crate::base::construction
// Provides: {"impl_704"}
// Dependencies: {}
impl < T , D : Dim > OMatrix < T , D , D > where T : Scalar , DefaultAllocator : Allocator < D , D > , { # [doc = " Creates a square matrix with its diagonal set to `diag` and all other entries set to 0."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::{Vector3, DVector, Matrix3, DMatrix};"] # [doc = " # use std::iter;"] # [doc = ""] # [doc = " let m = Matrix3::from_diagonal(&Vector3::new(1.0, 2.0, 3.0));"] # [doc = " // The two additional arguments represent the matrix dimensions."] # [doc = " let dm = DMatrix::from_diagonal(&DVector::from_row_slice(&[1.0, 2.0, 3.0]));"] # [doc = ""] # [doc = " assert!(m.m11 == 1.0 && m.m12 == 0.0 && m.m13 == 0.0 &&"] # [doc = "         m.m21 == 0.0 && m.m22 == 2.0 && m.m23 == 0.0 &&"] # [doc = "         m.m31 == 0.0 && m.m32 == 0.0 && m.m33 == 3.0);"] # [doc = " assert!(dm[(0, 0)] == 1.0 && dm[(0, 1)] == 0.0 && dm[(0, 2)] == 0.0 &&"] # [doc = "         dm[(1, 0)] == 0.0 && dm[(1, 1)] == 2.0 && dm[(1, 2)] == 0.0 &&"] # [doc = "         dm[(2, 0)] == 0.0 && dm[(2, 1)] == 0.0 && dm[(2, 2)] == 3.0);"] # [doc = " ```"] # [inline] pub fn from_diagonal < SB : RawStorage < T , D > > (diag : & Vector < T , D , SB >) -> Self where T : Zero , { let (dim , _) = diag . shape_generic () ; let mut res = Self :: zeros_generic (dim , dim) ; for i in 0 .. diag . len () { unsafe { * res . get_unchecked_mut ((i , i)) = diag . vget_unchecked (i) . clone () ; } } res } }
};
}
