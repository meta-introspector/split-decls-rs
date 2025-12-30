// Generated macro for impl_707 (impl)
macro_rules! Depcrate_base_constructionimpl_707 {
() => {
// Module: crate::base::construction
// Provides: {"impl_707"}
// Dependencies: {}
# [doc = " # Constructors of matrices with a dynamic number of columns"] impl < T : Scalar , R : DimName > OMatrix < T , R , Dyn > where DefaultAllocator : Allocator < R , Dyn > , { impl_constructors ! (R , Dyn ; => R : DimName ; R :: name () , Dyn (ncols) ; ncols) ; }
};
}
