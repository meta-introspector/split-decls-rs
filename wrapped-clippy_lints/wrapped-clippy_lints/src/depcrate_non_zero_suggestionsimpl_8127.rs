// Generated macro for impl_8127 (impl)
macro_rules! Depcrate_non_zero_suggestionsimpl_8127 {
() => {
// Module: crate::non_zero_suggestions
// Provides: {"impl_8127"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for NonZeroSuggestions { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) { if let ExprKind :: Binary (op , _ , rhs) = expr . kind && matches ! (op . node , BinOpKind :: Div | BinOpKind :: Rem) { check_non_zero_conversion (cx , rhs , Applicability :: MachineApplicable) ; } else { let parent_is_binary = cx . tcx . hir_parent_iter (expr . hir_id) . any (| (_ , node) | { matches ! (node , rustc_hir :: Node :: Expr (parent_expr) if matches ! (parent_expr . kind , ExprKind :: Binary (..))) }) ; if ! parent_is_binary { check_non_zero_conversion (cx , expr , Applicability :: MaybeIncorrect) ; } } } }
};
}
