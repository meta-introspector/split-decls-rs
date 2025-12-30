// Generated macro for has_repr_attr (function)
macro_rules! Depcratehas_repr_attr {
() => {
// Module: crate
// Provides: {"has_repr_attr"}
// Dependencies: {}
pub fn has_repr_attr (cx : & LateContext < '_ > , hir_id : HirId) -> bool { find_attr ! (cx . tcx . hir_attrs (hir_id) , AttributeKind :: Repr { .. }) }
};
}
