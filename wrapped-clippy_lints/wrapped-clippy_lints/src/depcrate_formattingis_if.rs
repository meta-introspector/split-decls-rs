// Generated macro for is_if (function)
macro_rules! Depcrate_formattingis_if {
() => {
// Module: crate::formatting
// Provides: {"is_if"}
// Dependencies: {}
# [doc = " Check if the expression is an `if` or `if let`"] fn is_if (expr : & Expr) -> bool { matches ! (expr . kind , ExprKind :: If (..)) }
};
}
