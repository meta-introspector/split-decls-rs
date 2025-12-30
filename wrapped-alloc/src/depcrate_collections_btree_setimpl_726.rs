// Generated macro for impl_726 (impl)
macro_rules! Depcrate_collections_btree_setimpl_726 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_726"}
// Dependencies: {}
impl < T : Debug , A : Allocator + Clone > Debug for IntersectionInner < '_ , T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { IntersectionInner :: Stitch { a , b } => { f . debug_struct ("Stitch") . field ("a" , a) . field ("b" , b) . finish () } IntersectionInner :: Search { small_iter , large_set } => f . debug_struct ("Search") . field ("small_iter" , small_iter) . field ("large_set" , large_set) . finish () , IntersectionInner :: Answer (x) => f . debug_tuple ("Answer") . field (x) . finish () , } } }
};
}
