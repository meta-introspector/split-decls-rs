// Generated macro for DifferenceInner (enum)
macro_rules! Depcrate_collections_btree_setDifferenceInner {
() => {
// Module: crate::collections::btree::set
// Provides: {"DifferenceInner"}
// Dependencies: {}
enum DifferenceInner < 'a , T : 'a , A : Allocator + Clone > { Stitch { self_iter : Iter < 'a , T > , other_iter : Peekable < Iter < 'a , T > > , } , Search { self_iter : Iter < 'a , T > , other_set : & 'a BTreeSet < T , A > , } , Iterate (Iter < 'a , T >) , }
};
}
