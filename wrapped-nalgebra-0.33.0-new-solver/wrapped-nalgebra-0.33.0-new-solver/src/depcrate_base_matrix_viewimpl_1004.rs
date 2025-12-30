// Generated macro for impl_1004 (impl)
macro_rules! Depcrate_base_matrix_viewimpl_1004 {
() => {
// Module: crate::base::matrix_view
// Provides: {"impl_1004"}
// Dependencies: {}
impl < T , R , C , S > Matrix < T , R , C , S > where R : Dim , C : Dim , S : RawStorage < T , R , C > , { # [doc = " Returns this matrix as a view."] # [doc = ""] # [doc = " The returned view type is generally ambiguous unless specified."] # [doc = " This is particularly useful when working with functions or methods that take"] # [doc = " matrix views as input."] # [doc = ""] # [doc = " # Panics"] # [doc = " Panics if the dimensions of the view and the matrix are not compatible and this cannot"] # [doc = " be proven at compile-time. This might happen, for example, when constructing a static"] # [doc = " view of size 3x3 from a dynamically sized matrix of dimension 5x5."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " use nalgebra::{DMatrixSlice, SMatrixView};"] # [doc = ""] # [doc = " fn consume_view(_: DMatrixSlice<f64>) {}"] # [doc = ""] # [doc = " let matrix = nalgebra::Matrix3::zeros();"] # [doc = " consume_view(matrix.as_view());"] # [doc = ""] # [doc = " let dynamic_view: DMatrixSlice<f64> = matrix.as_view();"] # [doc = " let static_view_from_dyn: SMatrixView<f64, 3, 3> = dynamic_view.as_view();"] # [doc = " ```"] pub fn as_view < RView , CView , RViewStride , CViewStride > (& self ,) -> MatrixView < '_ , T , RView , CView , RViewStride , CViewStride > where RView : Dim , CView : Dim , RViewStride : Dim , CViewStride : Dim , ShapeConstraint : DimEq < R , RView > + DimEq < C , CView > + DimEq < RViewStride , S :: RStride > + DimEq < CViewStride , S :: CStride > , { self . into () } }
};
}
