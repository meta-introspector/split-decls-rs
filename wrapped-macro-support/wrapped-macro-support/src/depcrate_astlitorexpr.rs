// Generated macro for LitOrExpr (enum)
macro_rules! Depcrate_astLitOrExpr {
() => {
// Module: crate::ast
// Provides: {"LitOrExpr"}
// Dependencies: {}
# [doc = " An enum representing either a literal value (`Lit`) or an expression (`syn::Expr`)."] # [cfg_attr (feature = "extra-traits" , derive (Debug))] # [derive (Clone)] pub enum LitOrExpr { # [doc = " Represents an expression that needs to be evaluated before it can be encoded"] Expr (syn :: Expr) , # [doc = " Represents a literal string that can be directly encoded."] Lit (String) , }
};
}
