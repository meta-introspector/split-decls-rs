// Generated macro for is_questionmark_desugar_marked_call (function)
macro_rules! Depcrate_unit_types_unit_argis_questionmark_desugar_marked_call {
() => {
// Module: crate::unit_types::unit_arg
// Provides: {"is_questionmark_desugar_marked_call"}
// Dependencies: {}
fn is_questionmark_desugar_marked_call (expr : & Expr < '_ >) -> bool { use rustc_span :: hygiene :: DesugaringKind ; if let ExprKind :: Call (callee , _) = expr . kind { callee . span . is_desugaring (DesugaringKind :: QuestionMark) } else { false } }
};
}
