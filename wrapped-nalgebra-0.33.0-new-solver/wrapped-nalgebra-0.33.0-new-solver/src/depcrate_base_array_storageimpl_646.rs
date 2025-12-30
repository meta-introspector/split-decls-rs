// Generated macro for impl_646 (impl)
macro_rules! Depcrate_base_array_storageimpl_646 {
() => {
// Module: crate::base::array_storage
// Provides: {"impl_646"}
// Dependencies: {}
unsafe impl < T : Scalar , const R : usize , const C : usize > Storage < T , Const < R > , Const < C > > for ArrayStorage < T , R , C > where DefaultAllocator : Allocator < Const < R > , Const < C > , Buffer < T > = Self > , { # [inline] fn into_owned (self) -> Owned < T , Const < R > , Const < C > > where DefaultAllocator : Allocator < Const < R > , Const < C > > , { self } # [inline] fn clone_owned (& self) -> Owned < T , Const < R > , Const < C > > where DefaultAllocator : Allocator < Const < R > , Const < C > > , { self . clone () } # [inline] fn forget_elements (self) { std :: mem :: forget (self) ; } }
};
}
