// Generated macro for GlobalStateSnapshot (struct)
macro_rules! Depcrate_global_stateGlobalStateSnapshot {
() => {
// Module: crate::global_state
// Provides: {"GlobalStateSnapshot"}
// Dependencies: {}
# [doc = " An immutable snapshot of the world's state at a point in time."] pub (crate) struct GlobalStateSnapshot { pub (crate) config : Arc < Config > , pub (crate) analysis : Analysis , pub (crate) check_fixes : CheckFixes , mem_docs : MemDocs , pub (crate) semantic_tokens_cache : Arc < Mutex < FxHashMap < Url , SemanticTokens > > > , vfs : Arc < RwLock < (vfs :: Vfs , FxHashMap < FileId , LineEndings >) > > , pub (crate) workspaces : Arc < Vec < ProjectWorkspace > > , pub (crate) proc_macros_loaded : bool , pub (crate) flycheck : Arc < [FlycheckHandle] > , minicore : MiniCoreRustAnalyzerInternalOnly , }
};
}
