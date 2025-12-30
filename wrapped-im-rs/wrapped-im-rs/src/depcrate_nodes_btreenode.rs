// Generated macro for Node (struct)
macro_rules! Depcrate_nodes_btreeNode {
() => {
// Module: crate::nodes::btree
// Provides: {"Node"}
// Dependencies: {}
pub (crate) struct Node < A > { keys : Chunk < A , NodeSize > , children : Chunk < Option < PoolRef < Node < A > > > , Add1 < NodeSize > > , }
};
}
