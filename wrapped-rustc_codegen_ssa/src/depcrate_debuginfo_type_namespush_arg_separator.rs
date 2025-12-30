// Generated macro for push_arg_separator (function)
macro_rules! Depcrate_debuginfo_type_namespush_arg_separator {
() => {
// Module: crate::debuginfo::type_names
// Provides: {"push_arg_separator"}
// Dependencies: {}
fn push_arg_separator (cpp_like_debuginfo : bool , output : & mut String) { if cpp_like_debuginfo { output . push (',') ; } else { output . push_str (", ") ; } ; }
};
}
