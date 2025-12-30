// Generated macro for ExtractIfInner (struct)
macro_rules! Depcrate_collections_btree_mapExtractIfInner {
() => {
// Module: crate::collections::btree::map
// Provides: {"ExtractIfInner"}
// Dependencies: {}
# [doc = " Most of the implementation of ExtractIf are generic over the type"] # [doc = " of the predicate, thus also serving for BTreeSet::ExtractIf."] pub (super) struct ExtractIfInner < 'a , K , V , R > { # [doc = " Reference to the length field in the borrowed map, updated live."] length : & 'a mut usize , # [doc = " Buried reference to the root field in the borrowed map."] # [doc = " Wrapped in `Option` to allow drop handler to `take` it."] dormant_root : Option < DormantMutRef < 'a , Root < K , V > > > , # [doc = " Contains a leaf edge preceding the next element to be returned, or the last leaf edge."] # [doc = " Empty if the map has no root, if iteration went beyond the last leaf edge,"] # [doc = " or if a panic occurred in the predicate."] cur_leaf_edge : Option < Handle < NodeRef < marker :: Mut < 'a > , K , V , marker :: Leaf > , marker :: Edge > > , # [doc = " Range over which iteration was requested.  We don't need the left side, but we"] # [doc = " can't extract the right side without requiring K: Clone."] range : R , }
};
}
