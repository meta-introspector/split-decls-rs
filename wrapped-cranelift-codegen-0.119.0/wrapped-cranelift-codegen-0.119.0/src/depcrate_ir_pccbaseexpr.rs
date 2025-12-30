// Generated macro for BaseExpr (enum)
macro_rules! Depcrate_ir_pccBaseExpr {
() => {
// Module: crate::ir::pcc
// Provides: {"BaseExpr"}
// Dependencies: {}
# [doc = " The base part of a bound expression."] # [derive (Clone , Debug , Hash , PartialEq , Eq)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub enum BaseExpr { # [doc = " No dynamic part (i.e., zero)."] None , # [doc = " A global value."] GlobalValue (ir :: GlobalValue) , # [doc = " An SSA Value as a symbolic value. This can be referenced in"] # [doc = " facts even after we've lowered out of SSA: it becomes simply"] # [doc = " some symbolic value."] Value (ir :: Value) , # [doc = " Top of the address space. This is \"saturating\": the offset"] # [doc = " doesn't matter."] Max , }
};
}
