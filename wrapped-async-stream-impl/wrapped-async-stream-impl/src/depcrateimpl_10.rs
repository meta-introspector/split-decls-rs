// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl < T : Parse > Parse for Partial < T > { fn parse (input : ParseStream) -> Result < Self > { Ok (Partial (input . parse () ? , input . parse () ?)) } }
};
}
