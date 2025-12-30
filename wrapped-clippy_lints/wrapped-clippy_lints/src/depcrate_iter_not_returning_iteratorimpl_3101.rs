// Generated macro for impl_3101 (impl)
macro_rules! Depcrate_iter_not_returning_iteratorimpl_3101 {
() => {
// Module: crate::iter_not_returning_iterator
// Provides: {"impl_3101"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for IterNotReturningIterator { fn check_trait_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx TraitItem < '_ >) { if let TraitItemKind :: Fn (fn_sig , _) = & item . kind && matches ! (item . ident . name , sym :: iter | sym :: iter_mut) { check_sig (cx , item . ident . name , fn_sig , item . owner_id . def_id) ; } } fn check_impl_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx ImplItem < 'tcx >) { if let ImplItemKind :: Fn (fn_sig , _) = & item . kind && matches ! (item . ident . name , sym :: iter | sym :: iter_mut) && ! matches ! (cx . tcx . parent_hir_node (item . hir_id ()) , Node :: Item (Item { kind : ItemKind :: Impl (i) , .. }) if i . of_trait . is_some ()) { check_sig (cx , item . ident . name , fn_sig , item . owner_id . def_id) ; } } }
};
}
