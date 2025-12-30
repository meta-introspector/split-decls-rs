// Generated macro for IntersectionInner (enum)
macro_rules! Depcrate_collections_btree_setIntersectionInner {
() => {
// Module: crate::collections::btree::set
// Provides: {"IntersectionInner"}
// Dependencies: {}
enum IntersectionInner < 'a , T : 'a , A : Allocator + Clone > { Stitch { a : Iter < 'a , T > , b : Iter < 'a , T > , } , Search { small_iter : Iter < 'a , T > , large_set : & 'a BTreeSet < T , A > , } , Answer (Option < & 'a T >) , }
};
}
