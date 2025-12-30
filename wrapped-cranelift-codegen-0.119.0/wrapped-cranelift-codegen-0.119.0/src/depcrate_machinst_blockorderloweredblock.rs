// Generated macro for LoweredBlock (enum)
macro_rules! Depcrate_machinst_blockorderLoweredBlock {
() => {
// Module: crate::machinst::blockorder
// Provides: {"LoweredBlock"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub enum LoweredBlock { # [doc = " Block in original CLIF."] Orig { # [doc = " Original CLIF block."] block : Block , } , # [doc = " Critical edge between two CLIF blocks."] CriticalEdge { # [doc = " The predecessor block."] pred : Block , # [doc = " The successor block."] succ : Block , # [doc = " The index of this branch in the successor edges from `pred`, following the same"] # [doc = " indexing order as `inst_predicates::visit_block_succs`. This is used to distinguish"] # [doc = " multiple edges between the same CLIF blocks."] succ_idx : u32 , } , }
};
}
