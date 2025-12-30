// Generated macro for LibraryRoots (struct)
macro_rules! Depcrate_symbol_indexLibraryRoots {
() => {
// Module: crate::symbol_index
// Provides: {"LibraryRoots"}
// Dependencies: {}
# [doc = " The set of roots for crates.io libraries."] # [doc = " Files in libraries are assumed to never change."] # [salsa :: input (singleton , debug)] pub struct LibraryRoots { # [returns (ref)] pub roots : FxHashSet < SourceRootId > , }
};
}
