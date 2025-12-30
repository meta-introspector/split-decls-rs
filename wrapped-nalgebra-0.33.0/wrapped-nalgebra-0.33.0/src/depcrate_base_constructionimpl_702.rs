// Generated macro for impl_702 (impl)
macro_rules! Depcrate_base_constructionimpl_702 {
() => {
// Module: crate::base::construction
// Provides: {"impl_702"}
// Dependencies: {}
impl < T : Scalar , R : Dim , C : Dim > UninitMatrix < T , R , C > where DefaultAllocator : Allocator < R , C > , { # [doc = " Builds a matrix with uninitialized elements of type `MaybeUninit<T>`."] # [inline (always)] pub fn uninit (nrows : R , ncols : C) -> Self { unsafe { Self :: from_data_statically_unchecked (DefaultAllocator :: allocate_uninit (nrows , ncols)) } } }
};
}
