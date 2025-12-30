// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl Parse for ExportKey { fn parse (input : ParseStream < '_ >) -> Result < Self > { let l = input . lookahead1 () ; Ok (if l . peek (kw :: world) { input . parse :: < kw :: world > () ? ; Self :: World } else { Self :: Name (input . parse () ?) }) } }
};
}
