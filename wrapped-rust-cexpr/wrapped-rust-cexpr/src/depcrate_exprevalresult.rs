// Generated macro for EvalResult (enum)
macro_rules! Depcrate_exprEvalResult {
() => {
// Module: crate::expr
// Provides: {"EvalResult"}
// Dependencies: {}
# [doc = " The result of parsing a literal or evaluating an expression."] # [derive (Debug , Clone , PartialEq)] # [allow (missing_docs)] pub enum EvalResult { Int (Wrapping < i64 >) , Float (f64) , Char (CChar) , Str (Vec < u8 >) , Invalid , }
};
}
