// Generated macro for extract_clone_suggestions (function)
macro_rules! Depcrate_needless_pass_by_valueextract_clone_suggestions {
() => {
// Module: crate::needless_pass_by_value
// Provides: {"extract_clone_suggestions"}
// Dependencies: {}
fn extract_clone_suggestions < 'tcx > (cx : & LateContext < 'tcx > , id : HirId , replace : & [(Symbol , & 'static str)] , body : & 'tcx Body < '_ > ,) -> Option < Vec < (Span , Cow < 'static , str >) > > { let mut spans = Vec :: new () ; for_each_expr_without_closures (body , | e | { if let ExprKind :: MethodCall (seg , recv , [] , _) = e . kind && recv . res_local_id () == Some (id) { if seg . ident . name == sym :: capacity { return ControlFlow :: Break (()) ; } for & (fn_name , suffix) in replace { if seg . ident . name == fn_name { spans . push ((e . span , snippet (cx , recv . span , "_") + suffix)) ; return ControlFlow :: Continue (Descend :: No) ; } } } ControlFlow :: Continue (Descend :: Yes) }) . is_none () . then_some (spans) }
};
}
