// Generated macro for OffendingFilterExpr (enum)
macro_rules! Depcrate_methods_filter_mapOffendingFilterExpr {
() => {
// Module: crate::methods::filter_map
// Provides: {"OffendingFilterExpr"}
// Dependencies: {}
# [derive (Debug , Copy , Clone)] enum OffendingFilterExpr < 'tcx > { # [doc = " `.filter(|opt| opt.is_some())`"] IsSome { # [doc = " The receiver expression"] receiver : & 'tcx Expr < 'tcx > , # [doc = " If `Some`, then this contains the span of an expression that possibly contains side"] # [doc = " effects: `.filter(|opt| side_effect(opt).is_some())`"] # [doc = "                         ^^^^^^^^^^^^^^^^"] # [doc = ""] # [doc = " We will use this later for warning the user that the suggested fix may change"] # [doc = " the behavior."] side_effect_expr_span : Option < Span > , } , # [doc = " `.filter(|res| res.is_ok())`"] IsOk { # [doc = " The receiver expression"] receiver : & 'tcx Expr < 'tcx > , # [doc = " See `IsSome`"] side_effect_expr_span : Option < Span > , } , # [doc = " `.filter(|enum| matches!(enum, Enum::A(_)))`"] Matches { # [doc = " The `DefId` of the variant being matched"] variant_def_id : hir :: def_id :: DefId , } , }
};
}
