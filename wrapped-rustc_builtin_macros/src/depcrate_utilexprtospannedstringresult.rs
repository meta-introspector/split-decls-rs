// Generated macro for ExprToSpannedStringResult (type)
macro_rules! Depcrate_utilExprToSpannedStringResult {
() => {
// Module: crate::util
// Provides: {"ExprToSpannedStringResult"}
// Dependencies: {}
# [doc = " `Ok` represents successfully retrieving the string literal at the correct"] # [doc = " position, e.g., `println(\"abc\")`."] pub (crate) type ExprToSpannedStringResult < 'a > = Result < ExprToSpannedString , UnexpectedExprKind < 'a > > ;
};
}
