// Generated macro for impl_927 (impl)
macro_rules! Depcrateimpl_927 {
() => {
// Module: crate
// Provides: {"impl_927"}
// Dependencies: {}
impl From < AssocItemId > for AttrDefId { fn from (assoc : AssocItemId) -> Self { match assoc { AssocItemId :: FunctionId (it) => AttrDefId :: FunctionId (it) , AssocItemId :: ConstId (it) => AttrDefId :: ConstId (it) , AssocItemId :: TypeAliasId (it) => AttrDefId :: TypeAliasId (it) , } } }
};
}
