// Generated macro for CheckResult (enum)
macro_rules! Depcrate_methods_filter_mapCheckResult {
() => {
// Module: crate::methods::filter_map
// Provides: {"CheckResult"}
// Dependencies: {}
# [doc = " The result of checking a `map` call, returned by `OffendingFilterExpr::check_map_call`"] # [derive (Debug)] enum CheckResult < 'tcx > { Method { map_arg : & 'tcx Expr < 'tcx > , # [doc = " The method that was called inside of `filter`"] method : CalledMethod , # [doc = " See `OffendingFilterExpr::IsSome`"] side_effect_expr_span : Option < Span > , } , PatternMatching { # [doc = " The span of the variant being matched"] # [doc = " if let Some(s) = enum"] # [doc = "        ^^^^^^^"] variant_span : Span , # [doc = " if let Some(s) = enum"] # [doc = "             ^"] variant_ident : Ident , } , }
};
}
