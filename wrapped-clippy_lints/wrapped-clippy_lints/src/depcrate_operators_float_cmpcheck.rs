// Generated macro for check (function)
macro_rules! Depcrate_operators_float_cmpcheck {
() => {
// Module: crate::operators::float_cmp
// Provides: {"check"}
// Dependencies: {}
pub (crate) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , op : BinOpKind , left : & 'tcx Expr < '_ > , right : & 'tcx Expr < '_ > ,) { if (op == BinOpKind :: Eq || op == BinOpKind :: Ne) && is_float (cx , left) { let ecx = ConstEvalCtxt :: new (cx) ; let ctxt = expr . span . ctxt () ; let left_is_local = match ecx . eval_with_source (left , ctxt) { Some ((c , s)) if ! is_allowed (& c) => s . is_local () , Some (_) => return , None => true , } ; let right_is_local = match ecx . eval_with_source (right , ctxt) { Some ((c , s)) if ! is_allowed (& c) => s . is_local () , Some (_) => return , None => true , } ; if is_signum (cx , left) && is_signum (cx , right) { return ; } if let Some (name) = parent_item_name (cx , expr) { let name = name . as_str () ; if name == "eq" || name == "ne" || name == "is_nan" || name . starts_with ("eq_") || name . ends_with ("_eq") { return ; } } let is_comparing_arrays = is_array (cx , left) || is_array (cx , right) ; let (lint , msg) = get_lint_and_message (left_is_local && right_is_local , is_comparing_arrays) ; span_lint_and_then (cx , lint , expr . span , msg , | diag | { let lhs = Sugg :: hir (cx , left , "..") ; let rhs = Sugg :: hir (cx , right , "..") ; if ! is_comparing_arrays { diag . span_suggestion (expr . span , "consider comparing them within some margin of error" , format ! ("({}).abs() {} error_margin" , lhs - rhs , if op == BinOpKind :: Eq { '<' } else { '>' }) , Applicability :: HasPlaceholders ,) ; } }) ; } }
};
}
