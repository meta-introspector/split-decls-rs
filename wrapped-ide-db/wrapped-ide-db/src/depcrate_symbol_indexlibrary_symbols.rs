// Generated macro for library_symbols (function)
macro_rules! Depcrate_symbol_indexlibrary_symbols {
() => {
// Module: crate::symbol_index
// Provides: {"library_symbols"}
// Dependencies: {}
fn library_symbols (db : & dyn SymbolsDatabase , source_root_id : SourceRootId) -> Arc < SymbolIndex > { let _p = tracing :: info_span ! ("library_symbols") . entered () ; let mut symbol_collector = SymbolCollector :: new (db) ; db . source_root_crates (source_root_id) . iter () . flat_map (| & krate | Crate :: from (krate) . modules (db)) . for_each (| module | symbol_collector . collect (module)) ; Arc :: new (SymbolIndex :: new (symbol_collector . finish ())) }
};
}
