// Generated macro for impl_286 (impl)
macro_rules! Depcrate_mappingimpl_286 {
() => {
// Module: crate::mapping
// Provides: {"impl_286"}
// Dependencies: {}
impl ToChalk for hir_def :: ImplId { type Chalk = chalk_db :: ImplId ; fn to_chalk (self , _db : & dyn HirDatabase) -> chalk_db :: ImplId { chalk_ir :: ImplId (self . as_id ()) } fn from_chalk (_db : & dyn HirDatabase , impl_id : chalk_db :: ImplId) -> hir_def :: ImplId { FromId :: from_id (impl_id . 0 . as_id ()) } }
};
}
