// Generated macro for impl_911 (impl)
macro_rules! Depcrateimpl_911 {
() => {
// Module: crate
// Provides: {"impl_911"}
// Dependencies: {}
impl From < AssocItemId > for GenericDefId { fn from (item : AssocItemId) -> Self { match item { AssocItemId :: FunctionId (f) => f . into () , AssocItemId :: ConstId (c) => c . into () , AssocItemId :: TypeAliasId (t) => t . into () , } } }
};
}
