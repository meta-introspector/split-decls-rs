// Generated macro for impl_647 (impl)
macro_rules! Depcrate_base_array_storageimpl_647 {
() => {
// Module: crate::base::array_storage
// Provides: {"impl_647"}
// Dependencies: {}
unsafe impl < T , const R : usize , const C : usize > RawStorageMut < T , Const < R > , Const < C > > for ArrayStorage < T , R , C > { # [inline] fn ptr_mut (& mut self) -> * mut T { self . 0 . as_mut_ptr () as * mut T } # [inline] unsafe fn as_mut_slice_unchecked (& mut self) -> & mut [T] { std :: slice :: from_raw_parts_mut (self . ptr_mut () , R * C) } }
};
}
