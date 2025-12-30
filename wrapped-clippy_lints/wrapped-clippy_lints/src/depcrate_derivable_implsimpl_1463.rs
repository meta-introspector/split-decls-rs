// Generated macro for impl_1463 (impl)
macro_rules! Depcrate_derivable_implsimpl_1463 {
() => {
// Module: crate::derivable_impls
// Provides: {"impl_1463"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for DerivableImpls { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < '_ >) { if let ItemKind :: Impl (Impl { of_trait : Some (of_trait) , items : [child] , self_ty , .. }) = item . kind && ! cx . tcx . is_automatically_derived (item . owner_id . to_def_id ()) && ! item . span . from_expansion () && let Some (def_id) = of_trait . trait_ref . trait_def_id () && cx . tcx . is_diagnostic_item (sym :: Default , def_id) && let impl_item_hir = child . hir_id () && let Node :: ImplItem (impl_item) = cx . tcx . hir_node (impl_item_hir) && let ImplItemKind :: Fn (_ , b) = & impl_item . kind && let Body { value : func_expr , .. } = cx . tcx . hir_body (* b) && let & ty :: Adt (adt_def , args) = cx . tcx . type_of (item . owner_id) . instantiate_identity () . kind () && let attrs = cx . tcx . hir_attrs (item . hir_id ()) && ! attrs . iter () . any (| attr | attr . doc_str () . is_some ()) && cx . tcx . hir_attrs (impl_item_hir) . is_empty () { let is_const = of_trait . constness == hir :: Constness :: Const ; if adt_def . is_struct () { check_struct (cx , item , self_ty , func_expr , adt_def , args , cx . tcx . typeck_body (* b) , is_const ,) ; } else if adt_def . is_enum () && self . msrv . meets (cx , msrvs :: DEFAULT_ENUM_ATTRIBUTE) { check_enum (cx , item , func_expr , adt_def , is_const) ; } } } }
};
}
