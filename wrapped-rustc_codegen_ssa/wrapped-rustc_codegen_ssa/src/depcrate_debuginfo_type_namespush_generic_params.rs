// Generated macro for push_generic_params (function)
macro_rules! Depcrate_debuginfo_type_namespush_generic_params {
() => {
// Module: crate::debuginfo::type_names
// Provides: {"push_generic_params"}
// Dependencies: {}
pub fn push_generic_params < 'tcx > (tcx : TyCtxt < 'tcx > , args : GenericArgsRef < 'tcx > , output : & mut String ,) { let _prof = tcx . prof . generic_activity ("compute_debuginfo_type_name") ; let mut visited = FxHashSet :: default () ; push_generic_params_internal (tcx , args , output , & mut visited) ; }
};
}
