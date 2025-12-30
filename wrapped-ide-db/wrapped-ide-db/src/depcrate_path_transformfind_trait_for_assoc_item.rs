// Generated macro for find_trait_for_assoc_item (function)
macro_rules! Depcrate_path_transformfind_trait_for_assoc_item {
() => {
// Module: crate::path_transform
// Provides: {"find_trait_for_assoc_item"}
// Dependencies: {}
fn find_trait_for_assoc_item (scope : & SemanticsScope < '_ > , type_param : hir :: TypeParam , assoc_item : ast :: NameRef ,) -> Option < hir :: Trait > { let db = scope . db ; let trait_bounds = type_param . trait_bounds (db) ; let assoc_item_name = assoc_item . text () ; for trait_ in trait_bounds { let names = trait_ . items (db) . into_iter () . filter_map (| item | match item { hir :: AssocItem :: TypeAlias (ta) => Some (ta . name (db)) , hir :: AssocItem :: Const (cst) => cst . name (db) , _ => None , }) ; for name in names { if assoc_item_name . as_str () == name . as_str () { return Some (trait_) ; } } } None }
};
}
