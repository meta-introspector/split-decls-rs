// Generated macro for impl_1003 (impl)
macro_rules! Depcrate_base_matrix_viewimpl_1003 {
() => {
// Module: crate::base::matrix_view
// Provides: {"impl_1003"}
// Dependencies: {}
impl < 'a , T , R , C , RStride , CStride > From < MatrixViewMut < 'a , T , R , C , RStride , CStride > > for MatrixView < 'a , T , R , C , RStride , CStride > where R : Dim , C : Dim , RStride : Dim , CStride : Dim , { fn from (view_mut : MatrixViewMut < 'a , T , R , C , RStride , CStride >) -> Self { let data = ViewStorage { ptr : view_mut . data . ptr , shape : view_mut . data . shape , strides : view_mut . data . strides , _phantoms : PhantomData , } ; unsafe { Matrix :: from_data_statically_unchecked (data) } } }
};
}
