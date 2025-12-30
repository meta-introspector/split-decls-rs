// Generated macro for impl_87 (impl)
macro_rules! Depcrate_verbatimimpl_87 {
() => {
// Module: crate::verbatim
// Provides: {"impl_87"}
// Dependencies: {}
impl Parse for VerbatimFn { fn parse (input : ParseStream) -> Result < Self > { Ok (VerbatimFn { attrs : input . call (Attribute :: parse_outer) ? , vis : input . parse () ? , defaultness : input . parse () ? , sig : input . parse () ? , semi_token : input . parse () ? , }) } }
};
}
