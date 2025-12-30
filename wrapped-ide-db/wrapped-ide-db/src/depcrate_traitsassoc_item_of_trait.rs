// Generated macro for assoc_item_of_trait (function)
macro_rules! Depcrate_traitsassoc_item_of_trait {
() => {
// Module: crate::traits
// Provides: {"assoc_item_of_trait"}
// Dependencies: {}
fn assoc_item_of_trait (db : & dyn HirDatabase , assoc : hir :: AssocItem , trait_ : hir :: Trait ,) -> Option < Definition > { use hir :: AssocItem :: * ; let name = match assoc { Function (it) => it . name (db) , Const (it) => it . name (db) ? , TypeAlias (it) => it . name (db) , } ; let item = trait_ . items (db) . into_iter () . find (| it | match (it , assoc) { (Function (trait_func) , Function (_)) => trait_func . name (db) == name , (Const (trait_konst) , Const (_)) => trait_konst . name (db) . map_or (false , | it | it == name) , (TypeAlias (trait_type_alias) , TypeAlias (_)) => trait_type_alias . name (db) == name , _ => false , }) ? ; Some (Definition :: from (item)) }
};
}
