// Generated macro for impl_self_ty_with_diagnostics_cycle_result (function)
macro_rules! Depcrate_lowerimpl_self_ty_with_diagnostics_cycle_result {
() => {
// Module: crate::lower
// Provides: {"impl_self_ty_with_diagnostics_cycle_result"}
// Dependencies: {}
pub (crate) fn impl_self_ty_with_diagnostics_cycle_result (db : & dyn HirDatabase , _impl_id : ImplId ,) -> (EarlyBinder < '_ , Ty < '_ > > , Diagnostics) { (EarlyBinder :: bind (Ty :: new_error (DbInterner :: new_with (db , None , None) , ErrorGuaranteed)) , None) }
};
}
