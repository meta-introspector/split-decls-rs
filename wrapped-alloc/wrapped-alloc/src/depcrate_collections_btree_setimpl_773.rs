// Generated macro for impl_773 (impl)
macro_rules! Depcrate_collections_btree_setimpl_773 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_773"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T : Ord , A : Allocator + Clone > Iterator for Intersection < 'a , T , A > { type Item = & 'a T ; fn next (& mut self) -> Option < & 'a T > { match & mut self . inner { IntersectionInner :: Stitch { a , b } => { let mut a_next = a . next () ? ; let mut b_next = b . next () ? ; loop { match a_next . cmp (b_next) { Less => a_next = a . next () ? , Greater => b_next = b . next () ? , Equal => return Some (a_next) , } } } IntersectionInner :: Search { small_iter , large_set } => loop { let small_next = small_iter . next () ? ; if large_set . contains (& small_next) { return Some (small_next) ; } } , IntersectionInner :: Answer (answer) => answer . take () , } } fn size_hint (& self) -> (usize , Option < usize >) { match & self . inner { IntersectionInner :: Stitch { a , b } => (0 , Some (min (a . len () , b . len ()))) , IntersectionInner :: Search { small_iter , .. } => (0 , Some (small_iter . len ())) , IntersectionInner :: Answer (None) => (0 , Some (0)) , IntersectionInner :: Answer (Some (_)) => (1 , Some (1)) , } } fn min (mut self) -> Option < & 'a T > { self . next () } }
};
}
