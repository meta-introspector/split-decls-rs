// Generated macro for has_c_repr_attr (function)
macro_rules! Depcrate_default_union_representationhas_c_repr_attr {
() => {
// Module: crate::default_union_representation
// Provides: {"has_c_repr_attr"}
// Dependencies: {}
fn has_c_repr_attr (cx : & LateContext < '_ > , hir_id : HirId) -> bool { let attrs = cx . tcx . hir_attrs (hir_id) ; find_attr ! (attrs , AttributeKind :: Repr { reprs , .. } if reprs . iter () . any (| (x , _) | * x == ReprAttr :: ReprC)) }
};
}
