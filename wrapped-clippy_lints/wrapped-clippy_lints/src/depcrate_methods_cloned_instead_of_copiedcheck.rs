// Generated macro for check (function)
macro_rules! Depcrate_methods_cloned_instead_of_copiedcheck {
() => {
// Module: crate::methods::cloned_instead_of_copied
// Provides: {"check"}
// Dependencies: {}
pub fn check (cx : & LateContext < '_ > , expr : & Expr < '_ > , recv : & Expr < '_ > , span : Span , msrv : Msrv) { let recv_ty = cx . typeck_results () . expr_ty_adjusted (recv) ; let inner_ty = match recv_ty . kind () { ty :: Adt (adt , subst) if cx . tcx . is_diagnostic_item (sym :: Option , adt . did ()) && msrv . meets (cx , msrvs :: OPTION_COPIED) => { subst . type_at (0) } , _ if cx . ty_based_def (expr) . opt_parent (cx) . is_diag_item (cx , sym :: Iterator) && msrv . meets (cx , msrvs :: ITERATOR_COPIED) => { match get_iterator_item_ty (cx , recv_ty) { Some (ty) => ty , _ => return , } } , _ => return , } ; match inner_ty . kind () { ty :: Ref (_ , ty , _) if is_copy (cx , * ty) => { } , _ => return , } span_lint_and_sugg (cx , CLONED_INSTEAD_OF_COPIED , span , "used `cloned` where `copied` could be used instead" , "try" , "copied" . into () , Applicability :: MachineApplicable ,) ; }
};
}
