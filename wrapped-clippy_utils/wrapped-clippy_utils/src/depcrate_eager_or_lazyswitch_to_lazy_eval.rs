// Generated macro for switch_to_lazy_eval (function)
macro_rules! Depcrate_eager_or_lazyswitch_to_lazy_eval {
() => {
// Module: crate::eager_or_lazy
// Provides: {"switch_to_lazy_eval"}
// Dependencies: {}
# [doc = " Whether the given expression should be changed to evaluate lazily"] pub fn switch_to_lazy_eval < 'tcx > (cx : & '_ LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) -> bool { expr_eagerness (cx , expr) == EagernessSuggestion :: Lazy }
};
}
