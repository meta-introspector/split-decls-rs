// Generated macro for impl_289 (impl)
macro_rules! Depcrate_mappingimpl_289 {
() => {
// Module: crate::mapping
// Provides: {"impl_289"}
// Dependencies: {}
impl ToChalk for TypeAliasAsValue { type Chalk = chalk_db :: AssociatedTyValueId ; fn to_chalk (self , _db : & dyn HirDatabase) -> chalk_db :: AssociatedTyValueId { rust_ir :: AssociatedTyValueId (self . 0 . as_id ()) } fn from_chalk (_db : & dyn HirDatabase , assoc_ty_value_id : chalk_db :: AssociatedTyValueId ,) -> TypeAliasAsValue { TypeAliasAsValue (TypeAliasId :: from_id (assoc_ty_value_id . 0)) } }
};
}
