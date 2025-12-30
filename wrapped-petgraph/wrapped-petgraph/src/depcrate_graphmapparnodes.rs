// Generated macro for ParNodes (struct)
macro_rules! Depcrate_graphmapParNodes {
() => {
// Module: crate::graphmap
// Provides: {"ParNodes"}
// Dependencies: {}
# [doc = " A [ParallelIterator] over this graph's nodes."] # [cfg (feature = "rayon")] pub struct ParNodes < 'a , N > where N : NodeTrait + Send + Sync , { iter : ParKeys < 'a , N , Vec < (N , CompactDirection) > > , }
};
}
