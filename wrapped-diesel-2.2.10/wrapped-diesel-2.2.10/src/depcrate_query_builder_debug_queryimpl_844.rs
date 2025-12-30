// Generated macro for impl_844 (impl)
macro_rules! Depcrate_query_builder_debug_queryimpl_844 {
() => {
// Module: crate::query_builder::debug_query
// Provides: {"impl_844"}
// Dependencies: {}
impl < DB > Debug for DebugBinds < '_ , DB > where DB : Backend + Default , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let backend = DB :: default () ; let mut buffer = Vec :: new () ; let ast_pass = AstPass :: debug_binds (& mut buffer , & backend) ; self . query . walk_ast (ast_pass) . map_err (| _ | fmt :: Error) ? ; format_list (f , & buffer) } }
};
}
