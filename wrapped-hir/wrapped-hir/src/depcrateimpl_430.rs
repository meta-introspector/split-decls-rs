// Generated macro for impl_430 (impl)
macro_rules! Depcrateimpl_430 {
() => {
// Module: crate
// Provides: {"impl_430"}
// Dependencies: {}
impl AsAssocItem for DefWithBody { fn as_assoc_item (self , db : & dyn HirDatabase) -> Option < AssocItem > { match self { DefWithBody :: Function (it) => it . as_assoc_item (db) , DefWithBody :: Const (it) => it . as_assoc_item (db) , DefWithBody :: Static (_) | DefWithBody :: Variant (_) => None , } } }
};
}
