// Generated macro for compute_debuginfo_type_name (function)
macro_rules! Depcrate_debuginfo_type_namescompute_debuginfo_type_name {
() => {
// Module: crate::debuginfo::type_names
// Provides: {"compute_debuginfo_type_name"}
// Dependencies: {}
# [doc = " Compute the name of the type as it should be stored in debuginfo. Does not do"] # [doc = " any caching, i.e., calling the function twice with the same type will also do"] # [doc = " the work twice. The `qualified` parameter only affects the first level of the"] # [doc = " type name, further levels (i.e., type parameters) are always fully qualified."] pub fn compute_debuginfo_type_name < 'tcx > (tcx : TyCtxt < 'tcx > , t : Ty < 'tcx > , qualified : bool ,) -> String { let _prof = tcx . prof . generic_activity ("compute_debuginfo_type_name") ; let mut result = String :: with_capacity (64) ; let mut visited = FxHashSet :: default () ; push_debuginfo_type_name (tcx , t , qualified , & mut result , & mut visited) ; result }
};
}
