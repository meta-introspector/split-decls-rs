// Generated macro for symbol_visibility_to_gcc (function)
macro_rules! Depcrate_basesymbol_visibility_to_gcc {
() => {
// Module: crate::base
// Provides: {"symbol_visibility_to_gcc"}
// Dependencies: {}
# [cfg (feature = "master")] pub fn symbol_visibility_to_gcc (visibility : SymbolVisibility) -> gccjit :: Visibility { match visibility { SymbolVisibility :: Hidden => gccjit :: Visibility :: Hidden , SymbolVisibility :: Protected => gccjit :: Visibility :: Protected , SymbolVisibility :: Interposable => gccjit :: Visibility :: Default , } }
};
}
