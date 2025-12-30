// Generated macro for BlockPredecessor (struct)
macro_rules! Depcrate_flowgraphBlockPredecessor {
() => {
// Module: crate::flowgraph
// Provides: {"BlockPredecessor"}
// Dependencies: {}
# [doc = " A basic block denoted by its enclosing Block and last instruction."] # [derive (Debug , PartialEq , Eq)] pub struct BlockPredecessor { # [doc = " Enclosing Block key."] pub block : Block , # [doc = " Last instruction in the basic block."] pub inst : Inst , }
};
}
