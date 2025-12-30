// Generated macro for impl_171 (impl)
macro_rules! Depcrate_parse_fixtureimpl_171 {
() => {
// Module: crate::parse::fixture
// Provides: {"impl_171"}
// Dependencies: {}
impl Parse for FixtureData { fn parse (input : ParseStream) -> syn :: Result < Self > { if input . peek (Token ! [::]) { Ok (Default :: default ()) } else { Ok (Self { items : parse_vector_trailing_till_double_comma :: < _ , Token ! [,] > (input) ? , }) } } }
};
}
