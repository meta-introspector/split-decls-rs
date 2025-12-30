// Generated macro for impl_52 (impl)
macro_rules! Depcrate_chalk_dbimpl_52 {
() => {
// Module: crate::chalk_db
// Provides: {"impl_52"}
// Dependencies: {}
impl chalk_ir :: UnificationDatabase < Interner > for & dyn HirDatabase { fn fn_def_variance (& self , fn_def_id : chalk_ir :: FnDefId < Interner > ,) -> chalk_ir :: Variances < Interner > { HirDatabase :: fn_def_variance (* self , from_chalk (* self , fn_def_id)) } fn adt_variance (& self , adt_id : chalk_ir :: AdtId < Interner >) -> chalk_ir :: Variances < Interner > { HirDatabase :: adt_variance (* self , adt_id . 0) } }
};
}
