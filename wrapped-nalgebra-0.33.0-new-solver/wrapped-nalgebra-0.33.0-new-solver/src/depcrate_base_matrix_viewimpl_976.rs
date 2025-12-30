// Generated macro for impl_976 (impl)
macro_rules! Depcrate_base_matrix_viewimpl_976 {
() => {
// Module: crate::base::matrix_view
// Provides: {"impl_976"}
// Dependencies: {}
impl < 'a , T : Scalar , R : Dim , C : Dim , RStride : Dim , CStride : Dim > ViewStorageMut < 'a , T , R , C , RStride , CStride > where Self : RawStorageMut < T , R , C > + IsContiguous , { # [doc = " Extracts the original slice from this storage"] pub fn into_slice_mut (self) -> & 'a mut [T] { let (nrows , ncols) = self . shape () ; if nrows . value () != 0 && ncols . value () != 0 { let sz = self . linear_index (nrows . value () - 1 , ncols . value () - 1) ; unsafe { slice :: from_raw_parts_mut (self . ptr , sz + 1) } } else { unsafe { slice :: from_raw_parts_mut (self . ptr , 0) } } } }
};
}
