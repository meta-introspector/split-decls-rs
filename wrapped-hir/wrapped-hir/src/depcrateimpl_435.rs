// Generated macro for impl_435 (impl)
macro_rules! Depcrateimpl_435 {
() => {
// Module: crate
// Provides: {"impl_435"}
// Dependencies: {}
impl HasVisibility for AssocItem { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { match self { AssocItem :: Function (f) => f . visibility (db) , AssocItem :: Const (c) => c . visibility (db) , AssocItem :: TypeAlias (t) => t . visibility (db) , } } }
};
}
