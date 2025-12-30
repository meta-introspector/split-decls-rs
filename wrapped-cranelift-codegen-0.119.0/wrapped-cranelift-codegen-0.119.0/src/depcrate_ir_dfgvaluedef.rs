// Generated macro for ValueDef (enum)
macro_rules! Depcrate_ir_dfgValueDef {
() => {
// Module: crate::ir::dfg
// Provides: {"ValueDef"}
// Dependencies: {}
# [doc = " Where did a value come from?"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum ValueDef { # [doc = " Value is the n'th result of an instruction."] Result (Inst , usize) , # [doc = " Value is the n'th parameter to a block."] Param (Block , usize) , # [doc = " Value is a union of two other values."] Union (Value , Value) , }
};
}
