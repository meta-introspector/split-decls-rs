// Generated macro for path_contains_type_arguments (function)
macro_rules! Depcrate_resolvingpath_contains_type_arguments {
() => {
// Module: crate::resolving
// Provides: {"path_contains_type_arguments"}
// Dependencies: {}
# [doc = " Returns whether `path` or any of its qualifiers contains type arguments."] fn path_contains_type_arguments (path : Option < ast :: Path >) -> bool { if let Some (path) = path { if let Some (segment) = path . segment () && segment . generic_arg_list () . is_some () { cov_mark :: hit ! (type_arguments_within_path) ; return true ; } return path_contains_type_arguments (path . qualifier ()) ; } false }
};
}
