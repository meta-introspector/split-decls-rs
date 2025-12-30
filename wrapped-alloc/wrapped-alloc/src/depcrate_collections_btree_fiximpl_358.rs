// Generated macro for impl_358 (impl)
macro_rules! Depcrate_collections_btree_fiximpl_358 {
() => {
// Module: crate::collections::btree::fix
// Provides: {"impl_358"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > NodeRef < marker :: Mut < 'a > , K , V , marker :: LeafOrInternal > { # [doc = " Stocks up a possibly underfull node, and if that causes its parent node"] # [doc = " to shrink, stocks up the parent, recursively."] # [doc = " Returns `true` if it fixed the tree, `false` if it couldn't because the"] # [doc = " root node became empty."] # [doc = ""] # [doc = " This method does not expect ancestors to already be underfull upon entry"] # [doc = " and panics if it encounters an empty ancestor."] pub (super) fn fix_node_and_affected_ancestors < A : Allocator + Clone > (mut self , alloc : A ,) -> bool { loop { match self . fix_node_through_parent (alloc . clone ()) { Ok (Some (parent)) => self = parent . forget_type () , Ok (None) => return true , Err (_) => return false , } } } }
};
}
