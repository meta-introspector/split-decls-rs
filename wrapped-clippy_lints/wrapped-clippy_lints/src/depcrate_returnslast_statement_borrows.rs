// Generated macro for last_statement_borrows (function)
macro_rules! Depcrate_returnslast_statement_borrows {
() => {
// Module: crate::returns
// Provides: {"last_statement_borrows"}
// Dependencies: {}
fn last_statement_borrows < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) -> bool { for_each_expr (cx , expr , | e | { if let Some (def_id) = fn_def_id (cx , e) && cx . tcx . fn_sig (def_id) . instantiate_identity () . skip_binder () . output () . walk () . any (| arg | matches ! (arg . kind () , GenericArgKind :: Lifetime (re) if ! re . is_static ())) { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } }) . is_some () }
};
}
