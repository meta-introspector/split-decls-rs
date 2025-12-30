// Generated macro for contains_try (function)
macro_rules! Depcrate_visitorscontains_try {
() => {
// Module: crate::visitors
// Provides: {"contains_try"}
// Dependencies: {}
# [doc = " returns `true` if expr contains match expr desugared from try"] fn contains_try (expr : & Expr < '_ >) -> bool { for_each_expr_without_closures (expr , | e | { if matches ! (e . kind , ExprKind :: Match (_ , _ , hir :: MatchSource :: TryDesugar (_))) { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } }) . is_some () }
};
}
