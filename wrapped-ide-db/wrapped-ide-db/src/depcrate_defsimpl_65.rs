// Generated macro for impl_65 (impl)
macro_rules! Depcrate_defsimpl_65 {
() => {
// Module: crate::defs
// Provides: {"impl_65"}
// Dependencies: {}
impl From < AssocItem > for Definition { fn from (assoc_item : AssocItem) -> Self { match assoc_item { AssocItem :: Function (it) => Definition :: Function (it) , AssocItem :: Const (it) => Definition :: Const (it) , AssocItem :: TypeAlias (it) => Definition :: TypeAlias (it) , } } }
};
}
