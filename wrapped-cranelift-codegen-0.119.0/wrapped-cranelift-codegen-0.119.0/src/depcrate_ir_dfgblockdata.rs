// Generated macro for BlockData (struct)
macro_rules! Depcrate_ir_dfgBlockData {
() => {
// Module: crate::ir::dfg
// Provides: {"BlockData"}
// Dependencies: {}
# [doc = " Contents of a basic block."] # [doc = ""] # [doc = " Parameters on a basic block are values that dominate everything in the block. All"] # [doc = " branches to this block must provide matching arguments, and the arguments to the entry block must"] # [doc = " match the function arguments."] # [derive (Clone , PartialEq , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct BlockData { # [doc = " List of parameters to this block."] params : ValueList , }
};
}
