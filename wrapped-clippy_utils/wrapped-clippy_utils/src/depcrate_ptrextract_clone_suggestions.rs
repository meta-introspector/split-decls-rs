// Generated macro for extract_clone_suggestions (function)
macro_rules! Depcrate_ptrextract_clone_suggestions {
() => {
// Module: crate::ptr
// Provides: {"extract_clone_suggestions"}
// Dependencies: {}
fn extract_clone_suggestions < 'tcx > (cx : & LateContext < 'tcx > , id : HirId , replace : & [(Symbol , & 'static str)] , body : & 'tcx Body < '_ > ,) -> Option < Vec < (Span , Cow < 'static , str >) > > { let mut spans = Vec :: new () ; for_each_expr_without_closures (body , | e | { if let ExprKind :: MethodCall (seg , recv , [] , _) = e . kind && path_to_local_id (recv , id) { if seg . ident . name == sym :: capacity { return ControlFlow :: Break (()) ; } for & (fn_name , suffix) in replace { if seg . ident . name == fn_name { spans . push ((e . span , snippet (cx , recv . span , "_") + suffix)) ; return ControlFlow :: Continue (Descend :: No) ; } } } ControlFlow :: Continue (Descend :: Yes) }) . is_none () . then_some (spans) }
};
}
