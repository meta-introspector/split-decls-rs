// Generated macro for impl_1005 (impl)
macro_rules! Depcrate_base_matrix_viewimpl_1005 {
() => {
// Module: crate::base::matrix_view
// Provides: {"impl_1005"}
// Dependencies: {}
impl < T , R , C , S > Matrix < T , R , C , S > where R : Dim , C : Dim , S : RawStorageMut < T , R , C > , { # [doc = " Returns this matrix as a mutable view."] # [doc = ""] # [doc = " The returned view type is generally ambiguous unless specified."] # [doc = " This is particularly useful when working with functions or methods that take"] # [doc = " matrix views as input."] # [doc = ""] # [doc = " # Panics"] # [doc = " Panics if the dimensions of the view and the matrix are not compatible and this cannot"] # [doc = " be proven at compile-time. This might happen, for example, when constructing a static"] # [doc = " view of size 3x3 from a dynamically sized matrix of dimension 5x5."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " use nalgebra::{DMatrixViewMut, SMatrixViewMut};"] # [doc = ""] # [doc = " fn consume_view(_: DMatrixViewMut<f64>) {}"] # [doc = ""] # [doc = " let mut matrix = nalgebra::Matrix3::zeros();"] # [doc = " consume_view(matrix.as_view_mut());"] # [doc = ""] # [doc = " let mut dynamic_view: DMatrixViewMut<f64> = matrix.as_view_mut();"] # [doc = " let static_view_from_dyn: SMatrixViewMut<f64, 3, 3> = dynamic_view.as_view_mut();"] # [doc = " ```"] pub fn as_view_mut < RView , CView , RViewStride , CViewStride > (& mut self ,) -> MatrixViewMut < '_ , T , RView , CView , RViewStride , CViewStride > where RView : Dim , CView : Dim , RViewStride : Dim , CViewStride : Dim , ShapeConstraint : DimEq < R , RView > + DimEq < C , CView > + DimEq < RViewStride , S :: RStride > + DimEq < CViewStride , S :: CStride > , { self . into () } }
};
}
