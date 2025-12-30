// Generated macro for crate_symbols (function)
macro_rules! Depcrate_symbol_indexcrate_symbols {
() => {
// Module: crate::symbol_index
// Provides: {"crate_symbols"}
// Dependencies: {}
# [doc = " The symbol indices of modules that make up a given crate."] pub fn crate_symbols (db : & dyn HirDatabase , krate : Crate) -> Box < [& SymbolIndex] > { let _p = tracing :: info_span ! ("crate_symbols") . entered () ; krate . modules (db) . into_iter () . map (| module | SymbolIndex :: module_symbols (db , module)) . collect () }
};
}
