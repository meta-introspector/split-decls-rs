// Generated macro for CFGNode (struct)
macro_rules! Depcrate_flowgraphCFGNode {
() => {
// Module: crate::flowgraph
// Provides: {"CFGNode"}
// Dependencies: {}
# [doc = " A container for the successors and predecessors of some Block."] # [derive (Clone , Default)] struct CFGNode { # [doc = " Instructions that can branch or jump to this block."] # [doc = ""] # [doc = " This maps branch instruction -> predecessor block which is redundant since the block containing"] # [doc = " the branch instruction is available from the `layout.inst_block()` method. We store the"] # [doc = " redundant information because:"] # [doc = ""] # [doc = " 1. Many `pred_iter()` consumers want the block anyway, so it is handily available."] # [doc = " 2. The `invalidate_block_successors()` may be called *after* branches have been removed from"] # [doc = "    their block, but we still need to remove them form the old block predecessor map."] # [doc = ""] # [doc = " The redundant block stored here is always consistent with the CFG successor lists, even after"] # [doc = " the IR has been edited."] pub predecessors : bforest :: Map < Inst , Block > , # [doc = " Set of blocks that are the targets of branches and jumps in this block."] # [doc = " The set is ordered by block number, indicated by the `()` comparator type."] pub successors : bforest :: Set < Block > , }
};
}
