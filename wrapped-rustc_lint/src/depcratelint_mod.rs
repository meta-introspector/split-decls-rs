// Generated macro for lint_mod (function)
macro_rules! Depcratelint_mod {
() => {
// Module: crate
// Provides: {"lint_mod"}
// Dependencies: {}
fn lint_mod (tcx : TyCtxt < '_ > , module_def_id : LocalModDefId) { late_lint_mod (tcx , module_def_id , BuiltinCombinedModuleLateLintPass :: new ()) ; }
};
}
