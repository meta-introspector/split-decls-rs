// Generated macro for walk_foreign_item (function)
macro_rules! Depcrate_intravisitwalk_foreign_item {
() => {
// Module: crate::intravisit
// Provides: {"walk_foreign_item"}
// Dependencies: {}
pub fn walk_foreign_item < 'v , V : Visitor < 'v > > (visitor : & mut V , foreign_item : & 'v ForeignItem < 'v > ,) -> V :: Result { let ForeignItem { ident , kind , owner_id : _ , span : _ , vis_span : _ , has_delayed_lints : _ } = foreign_item ; try_visit ! (visitor . visit_id (foreign_item . hir_id ())) ; try_visit ! (visitor . visit_ident (* ident)) ; match * kind { ForeignItemKind :: Fn (ref sig , param_idents , ref generics) => { try_visit ! (visitor . visit_generics (generics)) ; try_visit ! (visitor . visit_fn_decl (sig . decl)) ; for ident in param_idents . iter () . copied () { visit_opt ! (visitor , visit_ident , ident) ; } } ForeignItemKind :: Static (ref typ , _ , _) => { try_visit ! (visitor . visit_ty_unambig (typ)) ; } ForeignItemKind :: Type => () , } V :: Result :: output () }
};
}
