// Generated macro for impl_46 (impl)
macro_rules! Depcrate_from_idimpl_46 {
() => {
// Module: crate::from_id
// Provides: {"impl_46"}
// Dependencies: {}
impl From < AssocItem > for GenericDefId { fn from (item : AssocItem) -> Self { match item { AssocItem :: Function (f) => f . id . into () , AssocItem :: Const (c) => c . id . into () , AssocItem :: TypeAlias (t) => t . id . into () , } } }
};
}
