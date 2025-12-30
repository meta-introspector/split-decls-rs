// Generated macro for BlockLoweringOrder (struct)
macro_rules! Depcrate_machinst_blockorderBlockLoweringOrder {
() => {
// Module: crate::machinst::blockorder
// Provides: {"BlockLoweringOrder"}
// Dependencies: {}
# [doc = " Mapping from CLIF BBs to VCode BBs."] # [derive (Debug)] pub struct BlockLoweringOrder { # [doc = " Lowered blocks, in BlockIndex order. Each block is some combination of"] # [doc = " (i) a CLIF block, and (ii) inserted crit-edge blocks before or after;"] # [doc = " see [LoweredBlock] for details."] lowered_order : Vec < LoweredBlock > , # [doc = " BlockIndex values for successors for all lowered blocks, indexing `lowered_order`."] lowered_succ_indices : Vec < BlockIndex > , # [doc = " Ranges in `lowered_succ_indices` giving the successor lists for each lowered"] # [doc = " block. Indexed by lowering-order index (`BlockIndex`)."] lowered_succ_ranges : Vec < (Option < Inst > , std :: ops :: Range < usize >) > , # [doc = " Cold blocks. These blocks are not reordered in the"] # [doc = " `lowered_order` above; the lowered order must respect RPO"] # [doc = " (uses after defs) in order for lowering to be"] # [doc = " correct. Instead, this set is used to provide `is_cold()`,"] # [doc = " which is used by VCode emission to sink the blocks at the last"] # [doc = " moment (when we actually emit bytes into the MachBuffer)."] cold_blocks : FxHashSet < BlockIndex > , # [doc = " Lowered blocks that are indirect branch targets."] indirect_branch_targets : FxHashSet < BlockIndex > , }
};
}
