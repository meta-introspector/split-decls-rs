// Generated macro for SymbolsDatabase (trait)
macro_rules! Depcrate_symbol_indexSymbolsDatabase {
() => {
// Module: crate::symbol_index
// Provides: {"SymbolsDatabase"}
// Dependencies: {}
# [query_group :: query_group] pub trait SymbolsDatabase : HirDatabase + SourceDatabase { # [doc = " The symbol index for a given module. These modules should only be in source roots that"] # [doc = " are inside local_roots."] # [salsa :: invoke_interned (module_symbols)] fn module_symbols (& self , module : Module) -> Arc < SymbolIndex > ; # [doc = " The symbol index for a given source root within library_roots."] # [salsa :: invoke_interned (library_symbols)] fn library_symbols (& self , source_root_id : SourceRootId) -> Arc < SymbolIndex > ; # [salsa :: transparent] # [doc = " The symbol indices of modules that make up a given crate."] fn crate_symbols (& self , krate : Crate) -> Box < [Arc < SymbolIndex >] > ; # [doc = " The set of \"local\" (that is, from the current workspace) roots."] # [doc = " Files in local roots are assumed to change frequently."] # [salsa :: input] fn local_roots (& self) -> Arc < FxHashSet < SourceRootId > > ; # [doc = " The set of roots for crates.io libraries."] # [doc = " Files in libraries are assumed to never change."] # [salsa :: input] fn library_roots (& self) -> Arc < FxHashSet < SourceRootId > > ; }
};
}
