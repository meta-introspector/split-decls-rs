// Generated macro for impl_1112 (impl)
macro_rules! Depcrate_base_vec_storageimpl_1112 {
() => {
// Module: crate::base::vec_storage
// Provides: {"impl_1112"}
// Dependencies: {}
unsafe impl < T : Scalar , R : DimName > Storage < T , R , Dyn > for VecStorage < T , R , Dyn > where DefaultAllocator : Allocator < R , Dyn , Buffer < T > = Self > , { # [inline] fn into_owned (self) -> Owned < T , R , Dyn > where DefaultAllocator : Allocator < R , Dyn > , { self } # [inline] fn clone_owned (& self) -> Owned < T , R , Dyn > where DefaultAllocator : Allocator < R , Dyn > , { self . clone () } # [inline] fn forget_elements (mut self) { unsafe { self . data . set_len (0) } ; } }
};
}
