// Generated macro for Expr (struct)
macro_rules! Depcrate_ir_pccExpr {
() => {
// Module: crate::ir::pcc
// Provides: {"Expr"}
// Dependencies: {}
# [doc = " A bound expression."] # [derive (Clone , Debug , Hash , PartialEq , Eq)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct Expr { # [doc = " The dynamic (base) part."] pub base : BaseExpr , # [doc = " The static (offset) part."] pub offset : i64 , }
};
}
