// Generated macro for impl_10509 (impl)
macro_rules! Depcrate_unnecessary_box_returnsimpl_10509 {
() => {
// Module: crate::unnecessary_box_returns
// Provides: {"impl_10509"}
// Dependencies: {}
impl LateLintPass < '_ > for UnnecessaryBoxReturns { fn check_trait_item (& mut self , cx : & LateContext < '_ > , item : & TraitItem < '_ >) { let TraitItemKind :: Fn (signature , _) = & item . kind else { return ; } ; self . check_fn_item (cx , signature . decl , item . owner_id . def_id , item . ident . name) ; } fn check_impl_item (& mut self , cx : & LateContext < '_ > , item : & rustc_hir :: ImplItem < '_ >) { let Node :: Item (parent) = cx . tcx . parent_hir_node (item . hir_id ()) else { return ; } ; let ItemKind :: Impl (parent) = parent . kind else { return } ; if parent . of_trait . is_some () { return ; } let ImplItemKind :: Fn (signature , ..) = & item . kind else { return ; } ; self . check_fn_item (cx , signature . decl , item . owner_id . def_id , item . ident . name) ; } fn check_item (& mut self , cx : & LateContext < '_ > , item : & Item < '_ >) { let ItemKind :: Fn { ident , sig , .. } = & item . kind else { return ; } ; self . check_fn_item (cx , sig . decl , item . owner_id . def_id , ident . name) ; } }
};
}
