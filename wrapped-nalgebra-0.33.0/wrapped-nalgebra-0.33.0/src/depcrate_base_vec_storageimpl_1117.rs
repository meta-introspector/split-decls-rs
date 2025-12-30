// Generated macro for impl_1117 (impl)
macro_rules! Depcrate_base_vec_storageimpl_1117 {
() => {
// Module: crate::base::vec_storage
// Provides: {"impl_1117"}
// Dependencies: {}
unsafe impl < T , R : DimName > RawStorageMut < T , R , Dyn > for VecStorage < T , R , Dyn > { # [inline] fn ptr_mut (& mut self) -> * mut T { self . data . as_mut_ptr () } # [inline] unsafe fn as_mut_slice_unchecked (& mut self) -> & mut [T] { & mut self . data [..] } }
};
}
