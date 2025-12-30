// Generated macro for try_parse_insert (function)
macro_rules! Depcrate_entrytry_parse_insert {
() => {
// Module: crate::entry
// Provides: {"try_parse_insert"}
// Dependencies: {}
# [doc = " Inspect the given expression and return details about the `insert` call."] # [doc = ""] # [doc = " If the given expression is not an `insert` call into a `BTreeMap` or a `HashMap`, return `None`."] fn try_parse_insert < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) -> Option < InsertExpr < 'tcx > > { if let ExprKind :: MethodCall (_ , map , [key , value] , _) = expr . kind { let id = cx . typeck_results () . type_dependent_def_id (expr . hir_id) ? ; if let Some (insert) = cx . tcx . get_diagnostic_name (id) && matches ! (insert , sym :: btreemap_insert | sym :: hashmap_insert) { Some (InsertExpr { map , key , value }) } else { None } } else { None } }
};
}
