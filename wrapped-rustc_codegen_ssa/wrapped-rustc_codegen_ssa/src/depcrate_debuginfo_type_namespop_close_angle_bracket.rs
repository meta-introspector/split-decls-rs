// Generated macro for pop_close_angle_bracket (function)
macro_rules! Depcrate_debuginfo_type_namespop_close_angle_bracket {
() => {
// Module: crate::debuginfo::type_names
// Provides: {"pop_close_angle_bracket"}
// Dependencies: {}
fn pop_close_angle_bracket (output : & mut String) { assert ! (output . ends_with ('>') , "'output' does not end with '>': {output}") ; output . pop () ; if output . ends_with (' ') { output . pop () ; } }
};
}
