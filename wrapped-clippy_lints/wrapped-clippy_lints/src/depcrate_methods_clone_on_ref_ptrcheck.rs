// Generated macro for check (function)
macro_rules! Depcrate_methods_clone_on_ref_ptrcheck {
() => {
// Module: crate::methods::clone_on_ref_ptr
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ > , receiver : & hir :: Expr < '_ >) { let obj_ty = cx . typeck_results () . expr_ty (receiver) . peel_refs () ; if let ty :: Adt (adt , subst) = obj_ty . kind () && let Some (name) = cx . tcx . get_diagnostic_name (adt . did ()) { let caller_type = match name { sym :: Rc => "std::rc::Rc" , sym :: Arc => "std::sync::Arc" , sym :: RcWeak => "std::rc::Weak" , sym :: ArcWeak => "std::sync::Weak" , _ => return , } ; span_lint_and_then (cx , CLONE_ON_REF_PTR , expr . span , "using `.clone()` on a ref-counted pointer" , | diag | { let mut app = Applicability :: Unspecified ; let snippet = snippet_with_context (cx , receiver . span , expr . span . ctxt () , ".." , & mut app) . 0 ; let generic = subst . type_at (0) ; if generic . is_suggestable (cx . tcx , true) { diag . span_suggestion (expr . span , "try" , format ! ("{caller_type}::<{generic}>::clone(&{snippet})") , app ,) ; } else { diag . span_suggestion (expr . span , "try" , format ! ("{caller_type}::</* generic */>::clone(&{snippet})") , Applicability :: HasPlaceholders ,) ; } } ,) ; } }
};
}
