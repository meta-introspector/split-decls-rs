// Generated macro for impl_2404 (impl)
macro_rules! Depcrate_from_over_intoimpl_2404 {
() => {
// Module: crate::from_over_into
// Provides: {"impl_2404"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for FromOverInto { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < '_ >) { if let ItemKind :: Impl (Impl { of_trait : Some (of_trait) , self_ty , items : [impl_item_ref] , .. }) = item . kind && let Some (into_trait_seg) = of_trait . trait_ref . path . segments . last () && let Some (GenericArgs { args : [GenericArg :: Type (target_ty)] , .. }) = into_trait_seg . args && span_is_local (item . span) && let middle_trait_ref = cx . tcx . impl_trait_ref (item . owner_id) . instantiate_identity () && cx . tcx . is_diagnostic_item (sym :: Into , middle_trait_ref . def_id) && ! matches ! (middle_trait_ref . args . type_at (1) . kind () , ty :: Alias (ty :: Opaque , _)) && self . msrv . meets (cx , msrvs :: RE_REBALANCING_COHERENCE) { span_lint_and_then (cx , FROM_OVER_INTO , cx . tcx . sess . source_map () . guess_head_span (item . span) , "an implementation of `From` is preferred since it gives you `Into<_>` for free where the reverse isn't true" , | diag | { if target_ty . peel_refs () . basic_res () . opt_def_id () . is_none_or (| id | ! id . is_local ()) { diag . help ("`impl From<Local> for Foreign` is allowed by the orphan rules, for more information see\n\
                            https://doc.rust-lang.org/reference/items/implementations.html#trait-implementation-coherence") ; } let message = format ! ("replace the `Into` implementation with `From<{}>`" , middle_trait_ref . self_ty ()) ; if let Some (suggestions) = convert_to_from (cx , into_trait_seg , target_ty . as_unambig_ty () , self_ty , * impl_item_ref) { diag . multipart_suggestion (message , suggestions , Applicability :: MachineApplicable) ; } else { diag . help (message) ; } } ,) ; } } }
};
}
