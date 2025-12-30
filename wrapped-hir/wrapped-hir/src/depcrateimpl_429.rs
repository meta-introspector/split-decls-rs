// Generated macro for impl_429 (impl)
macro_rules! Depcrateimpl_429 {
() => {
// Module: crate
// Provides: {"impl_429"}
// Dependencies: {}
impl AsAssocItem for ModuleDef { fn as_assoc_item (self , db : & dyn HirDatabase) -> Option < AssocItem > { match self { ModuleDef :: Function (it) => it . as_assoc_item (db) , ModuleDef :: Const (it) => it . as_assoc_item (db) , ModuleDef :: TypeAlias (it) => it . as_assoc_item (db) , _ => None , } } }
};
}
