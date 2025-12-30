// Generated macro for impl_1111 (impl)
macro_rules! Depcrate_base_vec_storageimpl_1111 {
() => {
// Module: crate::base::vec_storage
// Provides: {"impl_1111"}
// Dependencies: {}
unsafe impl < T , R : DimName > RawStorage < T , R , Dyn > for VecStorage < T , R , Dyn > { type RStride = U1 ; type CStride = R ; # [inline] fn ptr (& self) -> * const T { self . data . as_ptr () } # [inline] fn shape (& self) -> (R , Dyn) { (self . nrows , self . ncols) } # [inline] fn strides (& self) -> (Self :: RStride , Self :: CStride) { (Self :: RStride :: name () , self . nrows) } # [inline] fn is_contiguous (& self) -> bool { true } # [inline] unsafe fn as_slice_unchecked (& self) -> & [T] { & self . data } }
};
}
