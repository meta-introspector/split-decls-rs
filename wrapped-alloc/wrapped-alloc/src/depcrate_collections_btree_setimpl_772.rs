// Generated macro for impl_772 (impl)
macro_rules! Depcrate_collections_btree_setimpl_772 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_772"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , A : Allocator + Clone > Clone for Intersection < '_ , T , A > { fn clone (& self) -> Self { Intersection { inner : match & self . inner { IntersectionInner :: Stitch { a , b } => { IntersectionInner :: Stitch { a : a . clone () , b : b . clone () } } IntersectionInner :: Search { small_iter , large_set } => { IntersectionInner :: Search { small_iter : small_iter . clone () , large_set } } IntersectionInner :: Answer (answer) => IntersectionInner :: Answer (* answer) , } , } } }
};
}
