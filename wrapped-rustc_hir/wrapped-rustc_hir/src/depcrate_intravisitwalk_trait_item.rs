// Generated macro for walk_trait_item (function)
macro_rules! Depcrate_intravisitwalk_trait_item {
() => {
// Module: crate::intravisit
// Provides: {"walk_trait_item"}
// Dependencies: {}
pub fn walk_trait_item < 'v , V : Visitor < 'v > > (visitor : & mut V , trait_item : & 'v TraitItem < 'v > ,) -> V :: Result { let TraitItem { ident , generics , ref defaultness , ref kind , span , owner_id : _ , has_delayed_lints : _ , } = * trait_item ; let hir_id = trait_item . hir_id () ; try_visit ! (visitor . visit_ident (ident)) ; try_visit ! (visitor . visit_generics (& generics)) ; try_visit ! (visitor . visit_defaultness (& defaultness)) ; try_visit ! (visitor . visit_id (hir_id)) ; match * kind { TraitItemKind :: Const (ref ty , default) => { try_visit ! (visitor . visit_ty_unambig (ty)) ; visit_opt ! (visitor , visit_nested_body , default) ; } TraitItemKind :: Fn (ref sig , TraitFn :: Required (param_idents)) => { try_visit ! (visitor . visit_fn_decl (sig . decl)) ; for ident in param_idents . iter () . copied () { visit_opt ! (visitor , visit_ident , ident) ; } } TraitItemKind :: Fn (ref sig , TraitFn :: Provided (body_id)) => { try_visit ! (visitor . visit_fn (FnKind :: Method (ident , sig) , sig . decl , body_id , span , trait_item . owner_id . def_id ,)) ; } TraitItemKind :: Type (bounds , ref default) => { walk_list ! (visitor , visit_param_bound , bounds) ; visit_opt ! (visitor , visit_ty_unambig , default) ; } } V :: Result :: output () }
};
}
