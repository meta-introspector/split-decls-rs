// Generated macro for is_try_block (function)
macro_rules! Depcrate_question_markis_try_block {
() => {
// Module: crate::question_mark
// Provides: {"is_try_block"}
// Dependencies: {}
fn is_try_block (cx : & LateContext < '_ > , bl : & Block < '_ >) -> bool { if let Some (expr) = bl . expr && let ExprKind :: Call (callee , [_]) = expr . kind && let ExprKind :: Path (qpath) = callee . kind && cx . tcx . qpath_is_lang_item (qpath , LangItem :: TryTraitFromOutput) { true } else { false } }
};
}
