// Generated macro for LocalRoots (struct)
macro_rules! Depcrate_symbol_indexLocalRoots {
() => {
// Module: crate::symbol_index
// Provides: {"LocalRoots"}
// Dependencies: {}
# [doc = " The set of \"local\" (that is, from the current workspace) roots."] # [doc = " Files in local roots are assumed to change frequently."] # [salsa :: input (singleton , debug)] pub struct LocalRoots { # [returns (ref)] pub roots : FxHashSet < SourceRootId > , }
};
}
