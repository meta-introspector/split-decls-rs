// Generated macro for check_trait_items (function)
macro_rules! Depcrate_len_zerocheck_trait_items {
() => {
// Module: crate::len_zero
// Provides: {"check_trait_items"}
// Dependencies: {}
fn check_trait_items (cx : & LateContext < '_ > , visited_trait : & Item < '_ > , ident : Ident , trait_items : & [TraitItemId]) { fn is_named_self (cx : & LateContext < '_ > , item : TraitItemId , name : Symbol) -> bool { cx . tcx . item_name (item . owner_id) == name && matches ! (cx . tcx . fn_arg_idents (item . owner_id) , [Some (Ident { name : kw :: SelfLower , .. })] ,) } fn fill_trait_set (traitt : DefId , set : & mut DefIdSet , cx : & LateContext < '_ >) { if set . insert (traitt) { for supertrait in supertrait_def_ids (cx . tcx , traitt) { fill_trait_set (supertrait , set , cx) ; } } } if cx . effective_visibilities . is_exported (visited_trait . owner_id . def_id) && trait_items . iter () . any (| & i | is_named_self (cx , i , sym :: len)) { let mut current_and_super_traits = DefIdSet :: default () ; fill_trait_set (visited_trait . owner_id . to_def_id () , & mut current_and_super_traits , cx) ; let is_empty_method_found = current_and_super_traits . items () . flat_map (| & i | cx . tcx . associated_items (i) . filter_by_name_unhygienic (sym :: is_empty)) . any (| i | i . is_method () && cx . tcx . fn_sig (i . def_id) . skip_binder () . inputs () . skip_binder () . len () == 1) ; if ! is_empty_method_found { span_lint (cx , LEN_WITHOUT_IS_EMPTY , visited_trait . span , format ! ("trait `{}` has a `len` method but no (possibly inherited) `is_empty` method" , ident . name) ,) ; } } }
};
}
