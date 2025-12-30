// Generated macro for impl_706 (impl)
macro_rules! Depcrate_base_constructionimpl_706 {
() => {
// Module: crate::base::construction
// Provides: {"impl_706"}
// Dependencies: {}
# [doc = " # Constructors of statically-sized vectors or statically-sized matrices"] impl < T : Scalar , R : DimName , C : DimName > OMatrix < T , R , C > where DefaultAllocator : Allocator < R , C > , { impl_constructors ! (R , C ; => R : DimName , => C : DimName ; R :: name () , C :: name () ;) ; }
};
}
