// Generated macro for check (function)
macro_rules! Depcrate_methods_suspicious_splitncheck {
() => {
// Module: crate::methods::suspicious_splitn
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , method_name : Symbol , expr : & Expr < '_ > , self_arg : & Expr < '_ > , count : u128) { if count <= 1 && let Some (call_id) = cx . typeck_results () . type_dependent_def_id (expr . hir_id) && let Some (impl_id) = cx . tcx . inherent_impl_of_assoc (call_id) && let self_ty = cx . tcx . type_of (impl_id) . instantiate_identity () && (self_ty . is_slice () || self_ty . is_str ()) { if matches ! (self_arg . kind , ExprKind :: Array ([])) || matches ! (self_arg . kind , ExprKind :: Lit (Spanned { node : LitKind :: Str (s , _) , .. }) if s . is_empty ()) { return ; } let (msg , note_msg) = if count == 0 { (format ! ("`{method_name}` called with `0` splits") , "the resulting iterator will always return `None`" ,) } else { (format ! ("`{method_name}` called with `1` split") , if self_ty . is_slice () { "the resulting iterator will always return the entire slice followed by `None`" } else { "the resulting iterator will always return the entire string followed by `None`" } ,) } ; span_lint_and_note (cx , SUSPICIOUS_SPLITN , expr . span , msg , None , note_msg) ; } }
};
}
