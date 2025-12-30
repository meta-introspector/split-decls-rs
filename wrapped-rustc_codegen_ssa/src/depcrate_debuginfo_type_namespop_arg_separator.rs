// Generated macro for pop_arg_separator (function)
macro_rules! Depcrate_debuginfo_type_namespop_arg_separator {
() => {
// Module: crate::debuginfo::type_names
// Provides: {"pop_arg_separator"}
// Dependencies: {}
fn pop_arg_separator (output : & mut String) { if output . ends_with (' ') { output . pop () ; } assert ! (output . ends_with (',')) ; output . pop () ; }
};
}
