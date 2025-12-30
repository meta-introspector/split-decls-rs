// Generated macro for impl_287 (impl)
macro_rules! Depcrate_mappingimpl_287 {
() => {
// Module: crate::mapping
// Provides: {"impl_287"}
// Dependencies: {}
impl ToChalk for CallableDefId { type Chalk = FnDefId ; fn to_chalk (self , _db : & dyn HirDatabase) -> FnDefId { chalk_ir :: FnDefId (salsa :: plumbing :: AsId :: as_id (& self)) } fn from_chalk (db : & dyn HirDatabase , fn_def_id : FnDefId) -> CallableDefId { salsa :: plumbing :: FromIdWithDb :: from_id (fn_def_id . 0 , db . zalsa ()) } }
};
}
