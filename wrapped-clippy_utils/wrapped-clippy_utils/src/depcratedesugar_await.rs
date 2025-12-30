// Generated macro for desugar_await (function)
macro_rules! Depcratedesugar_await {
() => {
// Module: crate
// Provides: {"desugar_await"}
// Dependencies: {}
# [doc = " If `expr` is a desugared `.await`, return the original expression if it does not come from a"] # [doc = " macro expansion."] pub fn desugar_await < 'tcx > (expr : & 'tcx Expr < '_ >) -> Option < & 'tcx Expr < 'tcx > > { if let ExprKind :: Match (match_value , _ , MatchSource :: AwaitDesugar) = expr . kind && let ExprKind :: Call (_ , [into_future_arg]) = match_value . kind && let ctxt = expr . span . ctxt () && for_each_expr_without_closures (into_future_arg , | e | { walk_span_to_context (e . span , ctxt) . map_or (ControlFlow :: Break (()) , | _ | ControlFlow :: Continue (())) }) . is_none () { Some (into_future_arg) } else { None } }
};
}
