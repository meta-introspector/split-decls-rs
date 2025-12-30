// Generated macro for BlockSummary (struct)
macro_rules! Depcrate_remove_constant_phisBlockSummary {
() => {
// Module: crate::remove_constant_phis
// Provides: {"BlockSummary"}
// Dependencies: {}
# [doc = " For some block, a useful bundle of info.  The `Block` itself is not stored"] # [doc = " here since it will be the key in the associated `FxHashMap` -- see"] # [doc = " `summaries` below.  For the `SmallVec` tuning params: most blocks have"] # [doc = " few parameters, hence `4`.  And almost all blocks have either one or two"] # [doc = " successors, hence `2`."] # [derive (Clone , Debug , Default)] struct BlockSummary < 'a > { # [doc = " Formal parameters for this `Block`."] # [doc = ""] # [doc = " These values are from group A."] formals : & 'a [Value] , # [doc = " Each outgoing edge from this block."] # [doc = ""] # [doc = " We don't bother to include transfers that pass zero parameters"] # [doc = " since that makes more work for the solver for no purpose."] # [doc = ""] # [doc = " We optimize for the case where a branch instruction has up to two"] # [doc = " outgoing edges, as unconditional jumps and conditional branches are"] # [doc = " more prominent than br_table."] dests : SmallVec < [OutEdge < 'a > ; 2] > , }
};
}
