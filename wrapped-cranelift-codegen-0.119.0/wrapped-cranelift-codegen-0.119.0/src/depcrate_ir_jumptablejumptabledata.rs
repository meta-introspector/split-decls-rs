// Generated macro for JumpTableData (struct)
macro_rules! Depcrate_ir_jumptableJumpTableData {
() => {
// Module: crate::ir::jumptable
// Provides: {"JumpTableData"}
// Dependencies: {}
# [doc = " Contents of a jump table."] # [doc = ""] # [doc = " All jump tables use 0-based indexing and are densely populated."] # [doc = ""] # [doc = " The default block for the jump table is stored as the first element of the underlying vector."] # [doc = " It can be accessed through the `default_block` and `default_block_mut` functions. All blocks"] # [doc = " may be iterated using the `all_branches` and `all_branches_mut` functions, which will both"] # [doc = " iterate over the default block first."] # [derive (Debug , Clone , PartialEq , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct JumpTableData { table : Vec < BlockCall > , }
};
}
