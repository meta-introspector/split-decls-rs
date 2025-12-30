// Generated macro for impl_1110 (impl)
macro_rules! Depcrate_base_vec_storageimpl_1110 {
() => {
// Module: crate::base::vec_storage
// Provides: {"impl_1110"}
// Dependencies: {}
unsafe impl < T : Scalar , C : Dim > Storage < T , Dyn , C > for VecStorage < T , Dyn , C > where DefaultAllocator : Allocator < Dyn , C , Buffer < T > = Self > , { # [inline] fn into_owned (self) -> Owned < T , Dyn , C > where DefaultAllocator : Allocator < Dyn , C > , { self } # [inline] fn clone_owned (& self) -> Owned < T , Dyn , C > where DefaultAllocator : Allocator < Dyn , C > , { self . clone () } # [inline] fn forget_elements (mut self) { unsafe { self . data . set_len (0) } ; } }
};
}
