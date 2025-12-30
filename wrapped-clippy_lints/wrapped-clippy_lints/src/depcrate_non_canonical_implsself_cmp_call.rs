// Generated macro for self_cmp_call (function)
macro_rules! Depcrate_non_canonical_implsself_cmp_call {
() => {
// Module: crate::non_canonical_impls
// Provides: {"self_cmp_call"}
// Dependencies: {}
# [doc = " Returns whether this is any of `self.cmp(..)`, `Self::cmp(self, ..)` or `Ord::cmp(self, ..)`."] fn self_cmp_call < 'tcx > (cx : & LateContext < 'tcx > , typeck : & TypeckResults < 'tcx > , cmp_expr : & 'tcx Expr < 'tcx > , needs_fully_qualified : & mut bool ,) -> bool { match cmp_expr . kind { ExprKind :: Call (path , [_ , _]) => path . res (typeck) . is_diag_item (cx , sym :: ord_cmp_method) , ExprKind :: MethodCall (_ , recv , [_] , ..) => { let ExprKind :: Path (path) = recv . kind else { return false ; } ; if last_path_segment (& path) . ident . name != kw :: SelfLower { return false ; } * needs_fully_qualified = true ; typeck . type_dependent_def_id (cmp_expr . hir_id) . is_some_and (| def_id | cx . tcx . is_diagnostic_item (sym :: ord_cmp_method , def_id)) } , _ => false , } }
};
}
