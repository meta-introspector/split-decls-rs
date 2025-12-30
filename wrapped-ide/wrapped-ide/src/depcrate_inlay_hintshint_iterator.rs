// Generated macro for hint_iterator (function)
macro_rules! Depcrate_inlay_hintshint_iterator {
() => {
// Module: crate::inlay_hints
// Provides: {"hint_iterator"}
// Dependencies: {}
# [doc = " Checks if the type is an Iterator from std::iter and returns the iterator trait and the item type of the concrete iterator."] fn hint_iterator < 'db > (sema : & Semantics < 'db , RootDatabase > , famous_defs : & FamousDefs < '_ , 'db > , ty : & hir :: Type < 'db > ,) -> Option < (hir :: Trait , hir :: TypeAlias , hir :: Type < 'db >) > { let db = sema . db ; let strukt = ty . strip_references () . as_adt () ? ; let krate = strukt . module (db) . krate () ; if krate != famous_defs . core () ? { return None ; } let iter_trait = famous_defs . core_iter_Iterator () ? ; let iter_mod = famous_defs . core_iter () ? ; if ! (strukt . visibility (db) == hir :: Visibility :: Public && strukt . module (db) . path_to_root (db) . contains (& iter_mod)) { return None ; } if ty . impls_trait (db , iter_trait , & []) { let assoc_type_item = iter_trait . items (db) . into_iter () . find_map (| item | match item { hir :: AssocItem :: TypeAlias (alias) if alias . name (db) == sym :: Item => Some (alias) , _ => None , }) ? ; if let Some (ty) = ty . normalize_trait_assoc_type (db , & [] , assoc_type_item) { return Some ((iter_trait , assoc_type_item , ty)) ; } } None }
};
}
