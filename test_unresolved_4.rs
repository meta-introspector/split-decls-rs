// Test case for unresolved dependency: rustc_session::config::ErrorOutputType::default
// Similar matches found in symbol database:
// - rustc_lint::default_could_be_derived::mk_lint
// - rustc_interface::callbacks::use_rustc_query_system___dep_graph___dep_node___default_dep_kind_debug
// - rustc_builtin_macros::default::use_rustc_ast_as_ast

// Expected: use rustc_session::config::ErrorOutputType;
fn test_default() {
    // Call: rustc_session::config::ErrorOutputType::default;
    println!("Testing dependency resolution");
}
