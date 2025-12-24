#[llm_error_message(message = "Module file not found for: dummy_ast_node")]
#[llm_context(
    crate_name = "wrapped_rustc_expand_base_lib",
    module_name = "dummy_ast_node"
)]
pub struct wrapped_rustc_expand_base_lib_decls_module_not_found_dummy_ast_node;
