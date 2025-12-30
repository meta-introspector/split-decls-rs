// Generated macro for compare_impl_item (function)
macro_rules! Depcrate_check_compare_impl_itemcompare_impl_item {
() => {
// Module: crate::check::compare_impl_item
// Provides: {"compare_impl_item"}
// Dependencies: {}
# [doc = " Call the query `tcx.compare_impl_item()` directly instead."] pub (super) fn compare_impl_item (tcx : TyCtxt < '_ > , impl_item_def_id : LocalDefId ,) -> Result < () , ErrorGuaranteed > { let impl_item = tcx . associated_item (impl_item_def_id) ; let trait_item = tcx . associated_item (impl_item . expect_trait_impl () ?) ; let impl_trait_ref = tcx . impl_trait_ref (impl_item . container_id (tcx)) . unwrap () . instantiate_identity () ; debug ! (? impl_trait_ref) ; match impl_item . kind { ty :: AssocKind :: Fn { .. } => compare_impl_method (tcx , impl_item , trait_item , impl_trait_ref) , ty :: AssocKind :: Type { .. } => compare_impl_ty (tcx , impl_item , trait_item , impl_trait_ref) , ty :: AssocKind :: Const { .. } => { compare_impl_const (tcx , impl_item , trait_item , impl_trait_ref) } } }
};
}
