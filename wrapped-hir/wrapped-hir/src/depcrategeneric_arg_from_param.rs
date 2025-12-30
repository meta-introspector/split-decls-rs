// Generated macro for generic_arg_from_param (function)
macro_rules! Depcrategeneric_arg_from_param {
() => {
// Module: crate
// Provides: {"generic_arg_from_param"}
// Dependencies: {}
fn generic_arg_from_param (db : & dyn HirDatabase , id : TypeOrConstParamId) -> Option < GenericArg < '_ > > { let local_idx = hir_ty :: param_idx (db , id) ? ; let defaults = db . generic_defaults (id . parent) ; let ty = defaults . get (local_idx) ? ; Some (ty . instantiate_identity ()) }
};
}
