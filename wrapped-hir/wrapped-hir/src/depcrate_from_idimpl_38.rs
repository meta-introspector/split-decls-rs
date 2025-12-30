// Generated macro for impl_38 (impl)
macro_rules! Depcrate_from_idimpl_38 {
() => {
// Module: crate::from_id
// Provides: {"impl_38"}
// Dependencies: {}
impl From < AssocItemId > for AssocItem { fn from (def : AssocItemId) -> Self { match def { AssocItemId :: FunctionId (it) => AssocItem :: Function (it . into ()) , AssocItemId :: TypeAliasId (it) => AssocItem :: TypeAlias (it . into ()) , AssocItemId :: ConstId (it) => AssocItem :: Const (it . into ()) , } } }
};
}
