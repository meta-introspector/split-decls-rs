// Generated macro for scan_nodes_to_ix (function)
macro_rules! Depcrate_parsescan_nodes_to_ix {
() => {
// Module: crate::parse
// Provides: {"scan_nodes_to_ix"}
// Dependencies: {}
# [doc = " Skips forward within a block to a node which spans (ends inclusive) the given"] # [doc = " index into the source."] fn scan_nodes_to_ix (tree : & Tree < Item > , mut node : Option < TreeIndex > , ix : usize ,) -> Option < TreeIndex > { while let Some (node_ix) = node { if tree [node_ix] . item . end <= ix { node = tree [node_ix] . next ; } else { break ; } } node }
};
}
