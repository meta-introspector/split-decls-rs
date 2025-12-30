// Generated macro for check_pat_name_for_ty (function)
macro_rules! Depcrate_unusual_namescheck_pat_name_for_ty {
() => {
// Module: crate::unusual_names
// Provides: {"check_pat_name_for_ty"}
// Dependencies: {}
fn check_pat_name_for_ty (cx : & LateContext < '_ > , pat : & Pat < '_ > , ty : Ty < '_ > , kind : & str) { if let PatKind :: Binding (_ , _ , ident , _) = pat . kind { let ty = ty . peel_refs () ; for (usual_ty , ty_str , usual_names) in USUAL_NAMES { if usual_ty . matches_ty (cx , ty) && ! usual_names . contains (& ident . name) && ident . name != kw :: SelfLower && ! ident . name . as_str () . starts_with ('_') { let usual_names = usual_names . iter () . map (| name | format ! ("`{name}`")) . join (" or ") ; span_lint_and_help (cx , UNUSUAL_NAMES , ident . span , format ! ("unusual name for a {kind} of type `{ty_str}`") , None , format ! ("prefer using {usual_names}") ,) ; } } } }
};
}
