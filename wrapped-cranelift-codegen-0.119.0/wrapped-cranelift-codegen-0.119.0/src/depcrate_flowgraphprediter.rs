// Generated macro for PredIter (struct)
macro_rules! Depcrate_flowgraphPredIter {
() => {
// Module: crate::flowgraph
// Provides: {"PredIter"}
// Dependencies: {}
# [doc = " An iterator over block predecessors. The iterator type is `BlockPredecessor`."] # [doc = ""] # [doc = " Each predecessor is an instruction that branches to the block."] pub struct PredIter < 'a > (bforest :: MapIter < 'a , Inst , Block >) ;
};
}
