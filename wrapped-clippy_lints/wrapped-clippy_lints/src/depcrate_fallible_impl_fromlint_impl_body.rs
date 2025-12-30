// Generated macro for lint_impl_body (function)
macro_rules! Depcrate_fallible_impl_fromlint_impl_body {
() => {
// Module: crate::fallible_impl_from
// Provides: {"lint_impl_body"}
// Dependencies: {}
fn lint_impl_body (cx : & LateContext < '_ > , item_def_id : hir :: OwnerId , impl_span : Span) { use rustc_hir :: Expr ; use rustc_hir :: intravisit :: { self , Visitor } ; struct FindPanicUnwrap < 'a , 'tcx > { lcx : & 'a LateContext < 'tcx > , typeck_results : & 'tcx ty :: TypeckResults < 'tcx > , result : Vec < Span > , } impl < 'tcx > Visitor < 'tcx > for FindPanicUnwrap < '_ , 'tcx > { fn visit_expr (& mut self , expr : & 'tcx Expr < '_ >) { if let Some (macro_call) = root_macro_call_first_node (self . lcx , expr) && is_panic (self . lcx , macro_call . def_id) { self . result . push (expr . span) ; } if let Some (arglists) = method_chain_args (expr , & [sym :: unwrap]) { let receiver_ty = self . typeck_results . expr_ty (arglists [0] . 0) . peel_refs () ; if receiver_ty . is_diag_item (self . lcx , sym :: Option) || receiver_ty . is_diag_item (self . lcx , sym :: Result) { self . result . push (expr . span) ; } } intravisit :: walk_expr (self , expr) ; } } for impl_item in cx . tcx . associated_items (item_def_id) . filter_by_name_unhygienic_and_kind (sym :: from , ty :: AssocTag :: Fn) { let impl_item_def_id = impl_item . def_id . expect_local () ; let body = cx . tcx . hir_body_owned_by (impl_item_def_id) ; let mut fpu = FindPanicUnwrap { lcx : cx , typeck_results : cx . tcx . typeck (impl_item_def_id) , result : Vec :: new () , } ; fpu . visit_expr (body . value) ; if ! fpu . result . is_empty () { span_lint_and_then (cx , FALLIBLE_IMPL_FROM , impl_span , "consider implementing `TryFrom` instead" , move | diag | { diag . help ("`From` is intended for infallible conversions only. \
                        Use `TryFrom` if there's a possibility for the conversion to fail" ,) ; diag . span_note (fpu . result , "potential failure(s)") ; } ,) ; } } }
};
}
