// Generated macro for Expr (enum)
macro_rules! Depcrate_grammarExpr {
() => {
// Module: crate::grammar
// Provides: {"Expr"}
// Dependencies: {}
# [derive (Debug)] # [derive (Clone)] # [derive (PartialEq)] pub enum Expr { Variable (String) , Num (i32) , AddSub (Vec < AddTerm >) , MultDiv (Vec < MultTerm >) , }
};
}
