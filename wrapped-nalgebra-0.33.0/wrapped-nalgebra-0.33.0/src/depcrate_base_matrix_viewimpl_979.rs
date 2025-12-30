// Generated macro for impl_979 (impl)
macro_rules! Depcrate_base_matrix_viewimpl_979 {
() => {
// Module: crate::base::matrix_view
// Provides: {"impl_979"}
// Dependencies: {}
unsafe impl < 'a , T , R : Dim , C : Dim , RStride : Dim , CStride : Dim > RawStorageMut < T , R , C > for ViewStorageMut < 'a , T , R , C , RStride , CStride > { # [inline] fn ptr_mut (& mut self) -> * mut T { self . ptr } # [inline] unsafe fn as_mut_slice_unchecked (& mut self) -> & mut [T] { let (nrows , ncols) = self . shape () ; if nrows . value () != 0 && ncols . value () != 0 { let sz = self . linear_index (nrows . value () - 1 , ncols . value () - 1) ; slice :: from_raw_parts_mut (self . ptr , sz + 1) } else { slice :: from_raw_parts_mut (self . ptr , 0) } } }
};
}
