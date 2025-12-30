// Generated macro for splitpoint (function)
macro_rules! Depcrate_collections_btree_nodesplitpoint {
() => {
// Module: crate::collections::btree::node
// Provides: {"splitpoint"}
// Dependencies: {}
# [doc = " Given an edge index where we want to insert into a node filled to capacity,"] # [doc = " computes a sensible KV index of a split point and where to perform the insertion."] # [doc = " The goal of the split point is for its key and value to end up in a parent node;"] # [doc = " the keys, values and edges to the left of the split point become the left child;"] # [doc = " the keys, values and edges to the right of the split point become the right child."] fn splitpoint (edge_idx : usize) -> (usize , LeftOrRight < usize >) { debug_assert ! (edge_idx <= CAPACITY) ; match edge_idx { 0 .. EDGE_IDX_LEFT_OF_CENTER => (KV_IDX_CENTER - 1 , LeftOrRight :: Left (edge_idx)) , EDGE_IDX_LEFT_OF_CENTER => (KV_IDX_CENTER , LeftOrRight :: Left (edge_idx)) , EDGE_IDX_RIGHT_OF_CENTER => (KV_IDX_CENTER , LeftOrRight :: Right (0)) , _ => (KV_IDX_CENTER + 1 , LeftOrRight :: Right (edge_idx - (KV_IDX_CENTER + 1 + 1))) , } }
};
}
