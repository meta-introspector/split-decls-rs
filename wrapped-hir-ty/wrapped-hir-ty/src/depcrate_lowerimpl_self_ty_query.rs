// Generated macro for impl_self_ty_query (function)
macro_rules! Depcrate_lowerimpl_self_ty_query {
() => {
// Module: crate::lower
// Provides: {"impl_self_ty_query"}
// Dependencies: {}
pub (crate) fn impl_self_ty_query < 'db > (db : & 'db dyn HirDatabase , impl_id : ImplId ,) -> EarlyBinder < 'db , Ty < 'db > > { db . impl_self_ty_with_diagnostics (impl_id) . 0 }
};
}
