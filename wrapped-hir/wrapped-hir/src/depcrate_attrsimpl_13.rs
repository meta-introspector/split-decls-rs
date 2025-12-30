// Generated macro for impl_13 (impl)
macro_rules! Depcrate_attrsimpl_13 {
() => {
// Module: crate::attrs
// Provides: {"impl_13"}
// Dependencies: {}
impl HasAttrs for AssocItem { fn attrs (self , db : & dyn HirDatabase) -> AttrsWithOwner { match self { AssocItem :: Function (it) => it . attrs (db) , AssocItem :: Const (it) => it . attrs (db) , AssocItem :: TypeAlias (it) => it . attrs (db) , } } fn attr_id (self) -> AttrDefId { match self { AssocItem :: Function (it) => it . attr_id () , AssocItem :: Const (it) => it . attr_id () , AssocItem :: TypeAlias (it) => it . attr_id () , } } }
};
}
