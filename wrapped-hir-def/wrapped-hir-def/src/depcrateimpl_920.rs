// Generated macro for impl_920 (impl)
macro_rules! Depcrateimpl_920 {
() => {
// Module: crate
// Provides: {"impl_920"}
// Dependencies: {}
impl From < AssocItemId > for AttrDefId { fn from (assoc : AssocItemId) -> Self { match assoc { AssocItemId :: FunctionId (it) => AttrDefId :: FunctionId (it) , AssocItemId :: ConstId (it) => AttrDefId :: ConstId (it) , AssocItemId :: TypeAliasId (it) => AttrDefId :: TypeAliasId (it) , } } }
};
}
