// Generated macro for impl_6884 (impl)
macro_rules! Depcrate_methods_utilsimpl_6884 {
() => {
// Module: crate::methods::utils
// Provides: {"impl_6884"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for CloneOrCopyVisitor < '_ , 'tcx > { type NestedFilter = nested_filter :: OnlyBodies ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . cx . tcx } fn visit_expr (& mut self , expr : & 'tcx Expr < 'tcx >) { walk_expr (self , expr) ; if self . is_binding (expr) { if let Some (parent) = get_parent_expr (self . cx , expr) { match parent . kind { ExprKind :: AddrOf (BorrowKind :: Ref , Mutability :: Not , referent) => { if ! parent . span . from_expansion () { self . references_to_binding . push ((parent . span . until (referent . span) , String :: new ())) ; } return ; } , ExprKind :: MethodCall (.. , args , _) => { if args . iter () . all (| arg | ! self . is_binding (arg)) && let Some (method_def_id) = self . cx . typeck_results () . type_dependent_def_id (parent . hir_id) && let method_ty = self . cx . tcx . type_of (method_def_id) . instantiate_identity () && let self_ty = method_ty . fn_sig (self . cx . tcx) . input (0) . skip_binder () && matches ! (self_ty . kind () , ty :: Ref (_ , _ , Mutability :: Not)) { return ; } } , _ => { } , } } self . clone_or_copy_needed = true ; } } }
};
}
