// Generated macro for get_ascii_type (function)
macro_rules! Depcrate_manual_ignore_case_cmpget_ascii_type {
() => {
// Module: crate::manual_ignore_case_cmp
// Provides: {"get_ascii_type"}
// Dependencies: {}
fn get_ascii_type < 'a > (cx : & LateContext < 'a > , kind : rustc_hir :: ExprKind < '_ >) -> Option < (Span , MatchType < 'a >) > { if let MethodCall (path , expr , _ , _) = kind { let is_lower = match path . ident . name { sym :: to_ascii_lowercase => true , sym :: to_ascii_uppercase => false , _ => return None , } ; let ty_raw = cx . typeck_results () . expr_ty (expr) ; let ty = ty_raw . peel_refs () ; if needs_ref_to_cmp (cx , ty) || ty . is_str () || ty . is_slice () || matches ! (ty . opt_diag_name (cx) , Some (sym :: OsStr | sym :: OsString)) { return Some ((expr . span , ToAscii (is_lower , ty_raw))) ; } } else if let Lit (expr) = kind { return Some ((expr . span , Literal (expr . node))) ; } None }
};
}
