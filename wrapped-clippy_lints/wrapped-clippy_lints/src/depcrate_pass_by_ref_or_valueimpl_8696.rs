// Generated macro for impl_8696 (impl)
macro_rules! Depcrate_pass_by_ref_or_valueimpl_8696 {
() => {
// Module: crate::pass_by_ref_or_value
// Provides: {"impl_8696"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for PassByRefOrValue { fn check_trait_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx hir :: TraitItem < '_ >) { if item . span . from_expansion () { return ; } if let hir :: TraitItemKind :: Fn (method_sig , _) = & item . kind { self . check_poly_fn (cx , item . owner_id . def_id , method_sig . decl , None) ; } } fn check_fn (& mut self , cx : & LateContext < 'tcx > , kind : FnKind < 'tcx > , decl : & 'tcx FnDecl < '_ > , _body : & 'tcx Body < '_ > , span : Span , def_id : LocalDefId ,) { if span . from_expansion () { return ; } let hir_id = cx . tcx . local_def_id_to_hir_id (def_id) ; match kind { FnKind :: ItemFn (.. , header) => { if header . abi != ExternAbi :: Rust { return ; } let attrs = cx . tcx . hir_attrs (hir_id) ; if find_attr ! (attrs , AttributeKind :: Inline (InlineAttr :: Always , _)) { return ; } for a in attrs { if a . has_name (sym :: proc_macro_derive) { return ; } } } , FnKind :: Method (..) => () , FnKind :: Closure => return , } if let Node :: Item (item) = cx . tcx . parent_hir_node (hir_id) && matches ! (item . kind , ItemKind :: Impl (Impl { of_trait : Some (_) , .. }) | ItemKind :: Trait (..)) { return ; } self . check_poly_fn (cx , def_id , decl , Some (span)) ; } }
};
}
