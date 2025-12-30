// Generated macro for impl_709 (impl)
macro_rules! Depcrate_base_constructionimpl_709 {
() => {
// Module: crate::base::construction
// Provides: {"impl_709"}
// Dependencies: {}
# [doc = " # Constructors of fully dynamic matrices"] # [cfg (any (feature = "std" , feature = "alloc"))] impl < T : Scalar > OMatrix < T , Dyn , Dyn > where DefaultAllocator : Allocator < Dyn , Dyn > , { impl_constructors ! (Dyn , Dyn ; ; Dyn (nrows) , Dyn (ncols) ; nrows , ncols) ; }
};
}
