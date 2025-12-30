// Generated macro for check (function)
macro_rules! Depcrate_methods_str_splitncheck {
() => {
// Module: crate::methods::str_splitn
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , method_name : Symbol , expr : & Expr < '_ > , self_arg : & Expr < '_ > , pat_arg : & Expr < '_ > , count : u128 , msrv : Msrv ,) { if count < 2 || ! cx . typeck_results () . expr_ty_adjusted (self_arg) . peel_refs () . is_str () { return ; } let needless = | usage_kind | match usage_kind { IterUsageKind :: Nth (n) => count > n + 1 , IterUsageKind :: NextTuple => count > 2 , } ; let manual = count == 2 && msrv . meets (cx , msrvs :: STR_SPLIT_ONCE) ; match parse_iter_usage (cx , expr . span . ctxt () , cx . tcx . hir_parent_iter (expr . hir_id)) { Some (usage) if needless (usage . kind) => lint_needless (cx , method_name , expr , self_arg , pat_arg) , Some (usage) if manual => check_manual_split_once (cx , method_name , expr , self_arg , pat_arg , & usage) , None if manual => { check_manual_split_once_indirect (cx , method_name , expr , self_arg , pat_arg) ; } , _ => { } , } }
};
}
