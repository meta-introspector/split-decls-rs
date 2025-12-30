// Generated macro for impl_1109 (impl)
macro_rules! Depcrate_base_vec_storageimpl_1109 {
() => {
// Module: crate::base::vec_storage
// Provides: {"impl_1109"}
// Dependencies: {}
unsafe impl < T , C : Dim > RawStorage < T , Dyn , C > for VecStorage < T , Dyn , C > { type RStride = U1 ; type CStride = Dyn ; # [inline] fn ptr (& self) -> * const T { self . data . as_ptr () } # [inline] fn shape (& self) -> (Dyn , C) { (self . nrows , self . ncols) } # [inline] fn strides (& self) -> (Self :: RStride , Self :: CStride) { (Self :: RStride :: name () , self . nrows) } # [inline] fn is_contiguous (& self) -> bool { true } # [inline] unsafe fn as_slice_unchecked (& self) -> & [T] { & self . data } }
};
}
