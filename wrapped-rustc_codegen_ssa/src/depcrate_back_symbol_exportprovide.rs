// Generated macro for provide (function)
macro_rules! Depcrate_back_symbol_exportprovide {
() => {
// Module: crate::back::symbol_export
// Provides: {"provide"}
// Dependencies: {}
pub (crate) fn provide (providers : & mut Providers) { providers . reachable_non_generics = reachable_non_generics_provider ; providers . is_reachable_non_generic = is_reachable_non_generic_provider_local ; providers . exported_non_generic_symbols = exported_non_generic_symbols_provider_local ; providers . exported_generic_symbols = exported_generic_symbols_provider_local ; providers . upstream_monomorphizations = upstream_monomorphizations_provider ; providers . is_unreachable_local_definition = is_unreachable_local_definition_provider ; providers . upstream_drop_glue_for = upstream_drop_glue_for_provider ; providers . upstream_async_drop_glue_for = upstream_async_drop_glue_for_provider ; providers . wasm_import_module_map = wasm_import_module_map ; providers . extern_queries . is_reachable_non_generic = is_reachable_non_generic_provider_extern ; providers . extern_queries . upstream_monomorphizations_for = upstream_monomorphizations_for_provider ; }
};
}
