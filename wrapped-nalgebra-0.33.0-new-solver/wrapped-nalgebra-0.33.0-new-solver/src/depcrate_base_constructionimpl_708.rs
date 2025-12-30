// Generated macro for impl_708 (impl)
macro_rules! Depcrate_base_constructionimpl_708 {
() => {
// Module: crate::base::construction
// Provides: {"impl_708"}
// Dependencies: {}
# [doc = " # Constructors of dynamic vectors and matrices with a dynamic number of rows"] impl < T : Scalar , C : DimName > OMatrix < T , Dyn , C > where DefaultAllocator : Allocator < Dyn , C > , { impl_constructors ! (Dyn , C ; => C : DimName ; Dyn (nrows) , C :: name () ; nrows) ; }
};
}
