macro_rules! deps {
    () => {
        SymbolIndex!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl SymbolIndex { # [doc = " The symbol index for a given source root within library_roots."] pub fn library_symbols (db : & dyn HirDatabase , source_root_id : SourceRootId) -> & SymbolIndex { # [salsa :: interned] struct InternedSourceRootId { id : SourceRootId , } # [salsa :: tracked (returns (ref))] fn library_symbols (db : & dyn HirDatabase , source_root_id : InternedSourceRootId < '_ > ,) -> SymbolIndex { let _p = tracing :: info_span ! ("library_symbols") . entered () ; hir :: attach_db (db , | | { let mut symbol_collector = SymbolCollector :: new (db , true) ; db . source_root_crates (source_root_id . id (db)) . iter () . flat_map (| & krate | Crate :: from (krate) . modules (db)) . for_each (| module | symbol_collector . collect (module)) ; SymbolIndex :: new (symbol_collector . finish ()) }) } library_symbols (db , InternedSourceRootId :: new (db , source_root_id)) } # [doc = " The symbol index for a given module. These modules should only be in source roots that"] # [doc = " are inside local_roots."] pub fn module_symbols (db : & dyn HirDatabase , module : Module) -> & SymbolIndex { # [salsa :: interned] struct InternedModuleId { id : hir :: ModuleId , } # [salsa :: tracked (returns (ref))] fn module_symbols (db : & dyn HirDatabase , module : InternedModuleId < '_ >) -> SymbolIndex { let _p = tracing :: info_span ! ("module_symbols") . entered () ; hir :: attach_db (db , | | { let module : Module = module . id (db) . into () ; SymbolIndex :: new (SymbolCollector :: new_module (db , module , ! module . krate () . origin (db) . is_local () ,)) }) } module_symbols (db , InternedModuleId :: new (db , hir :: ModuleId :: from (module))) } }
    };
}

impl_206!();