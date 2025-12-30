// Generated macro for ControlFlowGraph (struct)
macro_rules! Depcrate_flowgraphControlFlowGraph {
() => {
// Module: crate::flowgraph
// Provides: {"ControlFlowGraph"}
// Dependencies: {}
# [doc = " The Control Flow Graph maintains a mapping of blocks to their predecessors"] # [doc = " and successors where predecessors are basic blocks and successors are"] # [doc = " basic blocks."] pub struct ControlFlowGraph { data : SecondaryMap < Block , CFGNode > , pred_forest : bforest :: MapForest < Inst , Block > , succ_forest : bforest :: SetForest < Block > , valid : bool , }
};
}
