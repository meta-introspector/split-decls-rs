// Generated macro for try_parse_contains (function)
macro_rules! Depcrate_entrytry_parse_contains {
() => {
// Module: crate::entry
// Provides: {"try_parse_contains"}
// Dependencies: {}
# [doc = " Inspect the given expression and return details about the `contains_key` check."] # [doc = ""] # [doc = " If the given expression is not a `contains_key` check against a `BTreeMap` or a `HashMap`,"] # [doc = " return `None`."] fn try_parse_contains < 'tcx > (cx : & LateContext < '_ > , expr : & 'tcx Expr < '_ >) -> Option < (MapType , ContainsExpr < 'tcx >) > { let mut negated = false ; let expr = peel_hir_expr_while (expr , | e | match e . kind { ExprKind :: Unary (UnOp :: Not , e) => { negated = ! negated ; Some (e) } , _ => None , }) ; if let ExprKind :: MethodCall (_ , map , [arg] , _) = expr . kind && let Expr { kind : ExprKind :: AddrOf (_ , _ , key) , span : key_span , .. } = arg && key_span . eq_ctxt (expr . span) { let id = cx . typeck_results () . type_dependent_def_id (expr . hir_id) ? ; let expr = ContainsExpr { negated , map , key , call_ctxt : expr . span . ctxt () , } ; match cx . tcx . get_diagnostic_name (id) { Some (sym :: btreemap_contains_key) => Some ((MapType :: BTree , expr)) , Some (sym :: hashmap_contains_key) => Some ((MapType :: Hash , expr)) , _ => None , } } else { None } }
};
}
