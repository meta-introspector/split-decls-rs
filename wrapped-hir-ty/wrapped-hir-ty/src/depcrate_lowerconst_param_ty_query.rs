// Generated macro for const_param_ty_query (function)
macro_rules! Depcrate_lowerconst_param_ty_query {
() => {
// Module: crate::lower
// Provides: {"const_param_ty_query"}
// Dependencies: {}
pub (crate) fn const_param_ty_query < 'db > (db : & 'db dyn HirDatabase , def : ConstParamId) -> Ty < 'db > { db . const_param_ty_with_diagnostics (def) . 0 }
};
}
