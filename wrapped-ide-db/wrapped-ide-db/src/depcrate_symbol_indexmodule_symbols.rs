// Generated macro for module_symbols (function)
macro_rules! Depcrate_symbol_indexmodule_symbols {
() => {
// Module: crate::symbol_index
// Provides: {"module_symbols"}
// Dependencies: {}
fn module_symbols (db : & dyn SymbolsDatabase , module : Module) -> Arc < SymbolIndex > { let _p = tracing :: info_span ! ("module_symbols") . entered () ; Arc :: new (SymbolIndex :: new (SymbolCollector :: new_module (db , module))) }
};
}
