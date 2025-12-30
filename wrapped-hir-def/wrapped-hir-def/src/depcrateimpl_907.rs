// Generated macro for impl_907 (impl)
macro_rules! Depcrateimpl_907 {
() => {
// Module: crate
// Provides: {"impl_907"}
// Dependencies: {}
impl From < AssocItemId > for ModuleDefId { fn from (item : AssocItemId) -> Self { match item { AssocItemId :: FunctionId (f) => f . into () , AssocItemId :: ConstId (c) => c . into () , AssocItemId :: TypeAliasId (t) => t . into () , } } }
};
}
