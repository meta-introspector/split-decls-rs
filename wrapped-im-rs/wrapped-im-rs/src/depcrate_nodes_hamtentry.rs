// Generated macro for Entry (enum)
macro_rules! Depcrate_nodes_hamtEntry {
() => {
// Module: crate::nodes::hamt
// Provides: {"Entry"}
// Dependencies: {}
pub (crate) enum Entry < A > { Value (A , HashBits) , Collision (Ref < CollisionNode < A > >) , Node (PoolRef < Node < A > >) , }
};
}
