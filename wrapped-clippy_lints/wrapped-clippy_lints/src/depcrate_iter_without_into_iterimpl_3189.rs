// Generated macro for impl_3189 (impl)
macro_rules! Depcrate_iter_without_into_iterimpl_3189 {
() => {
// Module: crate::iter_without_into_iter
// Provides: {"impl_3189"}
// Dependencies: {}
impl LateLintPass < '_ > for IterWithoutIntoIter { fn check_item (& mut self , cx : & LateContext < '_ > , item : & rustc_hir :: Item < '_ >) { if let ItemKind :: Impl (imp) = item . kind && let TyKind :: Ref (_ , self_ty_without_ref) = & imp . self_ty . kind && let Some (of_trait) = imp . of_trait && of_trait . trait_ref . trait_def_id () . is_some_and (| did | cx . tcx . is_diagnostic_item (sym :: IntoIterator , did)) && ! item . span . in_external_macro (cx . sess () . source_map ()) && let & ty :: Ref (_ , ty , mtbl) = cx . tcx . type_of (item . owner_id) . instantiate_identity () . kind () && let expected_method_name = match mtbl { Mutability :: Mut => sym :: iter_mut , Mutability :: Not => sym :: iter , } && ! deref_chain (cx , ty) . any (| ty | { ty . peel_refs () . is_slice () || get_adt_inherent_method (cx , ty , expected_method_name) . is_some () }) && let Some (iter_assoc_span) = cx . tcx . associated_items (item . owner_id) . filter_by_name_unhygienic_and_kind (sym :: IntoIter , ty :: AssocTag :: Type) . next () . map (| assoc_item | { cx . tcx . hir_node_by_def_id (assoc_item . def_id . expect_local ()) . expect_impl_item () . expect_type () . span }) && is_ty_exported (cx , ty) { span_lint_and_then (cx , INTO_ITER_WITHOUT_ITER , item . span , format ! ("`IntoIterator` implemented for a reference type without an `{expected_method_name}` method") , | diag | { let sugg = format ! ("
impl {self_ty_without_ref} {{
    fn {expected_method_name}({ref_self}self) -> {iter_ty} {{
        <{ref_self}Self as IntoIterator>::into_iter(self)
    }}
}}
" , self_ty_without_ref = snippet (cx , self_ty_without_ref . ty . span , "..") , ref_self = mtbl . ref_prefix_str () , iter_ty = snippet (cx , iter_assoc_span , "..") ,) ; diag . span_suggestion_verbose (item . span . shrink_to_lo () , format ! ("consider implementing `{expected_method_name}`") , sugg , Applicability :: Unspecified ,) ; } ,) ; } } fn check_impl_item (& mut self , cx : & LateContext < '_ > , item : & rustc_hir :: ImplItem < '_ >) { let item_did = item . owner_id . to_def_id () ; let (borrow_prefix , expected_implicit_self) = match item . ident . name { sym :: iter => ("&" , ImplicitSelfKind :: RefImm) , sym :: iter_mut => ("&mut " , ImplicitSelfKind :: RefMut) , _ => return , } ; if ! item . span . in_external_macro (cx . sess () . source_map ()) && let ImplItemKind :: Fn (sig , _) = item . kind && let FnRetTy :: Return (ret) = sig . decl . output && is_nameable_in_impl_trait (ret) && cx . tcx . generics_of (item_did) . is_own_empty () && sig . decl . implicit_self == expected_implicit_self && sig . decl . inputs . len () == 1 && let Some (imp) = get_parent_as_impl (cx . tcx , item . hir_id ()) && imp . of_trait . is_none () && let sig = cx . tcx . liberate_late_bound_regions (item_did , cx . tcx . fn_sig (item_did) . instantiate_identity ()) && let ref_ty = sig . inputs () [0] && let Some (into_iter_did) = cx . tcx . get_diagnostic_item (sym :: IntoIterator) && let Some (iterator_did) = cx . tcx . get_diagnostic_item (sym :: Iterator) && let ret_ty = sig . output () && implements_trait (cx , ret_ty , iterator_did , & []) && let Some (iter_ty) = make_normalized_projection (cx . tcx , cx . typing_env () , iterator_did , sym :: Item , [ret_ty] ,) && ! implements_trait (cx , ref_ty , into_iter_did , & []) && is_ty_exported (cx , ref_ty . peel_refs ()) { let self_ty_snippet = format ! ("{borrow_prefix}{}" , snippet (cx , imp . self_ty . span , "..")) ; span_lint_and_then (cx , ITER_WITHOUT_INTO_ITER , item . span , format ! ("`{}` method without an `IntoIterator` impl for `{self_ty_snippet}`" , item . ident) , | diag | { let span_behind_impl = cx . tcx . def_span (cx . tcx . parent_hir_id (item . hir_id ()) . owner . def_id) . shrink_to_lo () ; let sugg = format ! ("
impl IntoIterator for {self_ty_snippet} {{
    type Item = {iter_ty};
    type IntoIter = {ret_ty};
    fn into_iter(self) -> Self::IntoIter {{
        self.iter()
    }}
}}
") ; diag . span_suggestion_verbose (span_behind_impl , format ! ("consider implementing `IntoIterator` for `{self_ty_snippet}`") , sugg , Applicability :: Unspecified ,) ; } ,) ; } } }
};
}
