// Generated macro for impl_645 (impl)
macro_rules! Depcrate_base_array_storageimpl_645 {
() => {
// Module: crate::base::array_storage
// Provides: {"impl_645"}
// Dependencies: {}
unsafe impl < T , const R : usize , const C : usize > RawStorage < T , Const < R > , Const < C > > for ArrayStorage < T , R , C > { type RStride = Const < 1 > ; type CStride = Const < R > ; # [inline] fn ptr (& self) -> * const T { self . 0 . as_ptr () as * const T } # [inline] fn shape (& self) -> (Const < R > , Const < C >) { (Const , Const) } # [inline] fn strides (& self) -> (Self :: RStride , Self :: CStride) { (Const , Const) } # [inline] fn is_contiguous (& self) -> bool { true } # [inline] unsafe fn as_slice_unchecked (& self) -> & [T] { std :: slice :: from_raw_parts (self . ptr () , R * C) } }
};
}
