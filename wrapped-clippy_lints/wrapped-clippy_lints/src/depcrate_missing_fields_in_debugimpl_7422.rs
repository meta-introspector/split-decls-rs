// Generated macro for impl_7422 (impl)
macro_rules! Depcrate_missing_fields_in_debugimpl_7422 {
() => {
// Module: crate::missing_fields_in_debug
// Provides: {"impl_7422"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for MissingFieldsInDebug { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < 'tcx >) { if let ItemKind :: Impl (Impl { of_trait : Some (of_trait) , self_ty , .. }) = item . kind && let Res :: Def (DefKind :: Trait , trait_def_id) = of_trait . trait_ref . path . res && let TyKind :: Path (QPath :: Resolved (_ , self_path)) = & self_ty . kind && let Res :: Def (DefKind :: Struct | DefKind :: Enum | DefKind :: Union , self_path_did) = self_path . res && cx . tcx . is_diagnostic_item (sym :: Debug , trait_def_id) && ! cx . tcx . is_automatically_derived (item . owner_id . to_def_id ()) && ! item . span . from_expansion () && let Some (fmt_item) = cx . tcx . associated_items (item . owner_id) . filter_by_name_unhygienic (sym :: fmt) . next () && let body = cx . tcx . hir_body_owned_by (fmt_item . def_id . expect_local ()) && let ExprKind :: Block (block , _) = body . value . kind && let self_ty = cx . tcx . type_of (self_path_did) . skip_binder () . peel_refs () && let Some (self_adt) = self_ty . ty_adt_def () && let Some (self_def_id) = self_adt . did () . as_local () && let Node :: Item (self_item) = cx . tcx . hir_node_by_def_id (self_def_id) && let typeck_results = cx . tcx . typeck_body (body . id ()) && should_lint (cx , typeck_results , block) && let ItemKind :: Struct (_ , _ , data) = & self_item . kind { check_struct (cx , typeck_results , block , self_ty , item , data) ; } } }
};
}
