// Generated macro for impl_9669 (impl)
macro_rules! Depcrate_self_named_constructorsimpl_9669 {
() => {
// Module: crate::self_named_constructors
// Provides: {"impl_9669"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for SelfNamedConstructors { fn check_impl_item (& mut self , cx : & LateContext < 'tcx > , impl_item : & 'tcx ImplItem < '_ >) { match impl_item . kind { ImplItemKind :: Fn (ref sig , _) => { if sig . decl . implicit_self . has_implicit_self () { return ; } } , _ => return , } let parent = cx . tcx . hir_get_parent_item (impl_item . hir_id ()) . def_id ; let item = cx . tcx . hir_expect_item (parent) ; let self_ty = cx . tcx . type_of (item . owner_id) . instantiate_identity () ; let ret_ty = return_ty (cx , impl_item . owner_id) ; if matches ! (item . kind , ItemKind :: Impl (Impl { of_trait : Some (_) , .. })) { return ; } if let Some (self_adt) = self_ty . ty_adt_def () { if ! contains_adt_constructor (ret_ty , self_adt) { return ; } } else if ! ret_ty . contains (self_ty) { return ; } if let Some (self_def) = self_ty . ty_adt_def () && let Some (self_local_did) = self_def . did () . as_local () && let Node :: Item (x) = cx . tcx . hir_node_by_def_id (self_local_did) && let Some (type_ident) = x . kind . ident () && let type_name = type_ident . name . as_str () . to_lowercase () && (impl_item . ident . name . as_str () == type_name || impl_item . ident . name . as_str () . replace ('_' , "") == type_name) { span_lint (cx , SELF_NAMED_CONSTRUCTORS , impl_item . span , format ! ("constructor `{}` has the same name as the type" , impl_item . ident . name) ,) ; } } }
};
}
