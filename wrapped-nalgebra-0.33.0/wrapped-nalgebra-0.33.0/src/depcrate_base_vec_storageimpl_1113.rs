// Generated macro for impl_1113 (impl)
macro_rules! Depcrate_base_vec_storageimpl_1113 {
() => {
// Module: crate::base::vec_storage
// Provides: {"impl_1113"}
// Dependencies: {}
unsafe impl < T , C : Dim > RawStorageMut < T , Dyn , C > for VecStorage < T , Dyn , C > { # [inline] fn ptr_mut (& mut self) -> * mut T { self . data . as_mut_ptr () } # [inline] unsafe fn as_mut_slice_unchecked (& mut self) -> & mut [T] { & mut self . data [..] } }
};
}
