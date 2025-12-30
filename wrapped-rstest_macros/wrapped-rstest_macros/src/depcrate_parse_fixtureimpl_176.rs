// Generated macro for impl_176 (impl)
macro_rules! Depcrate_parse_fixtureimpl_176 {
() => {
// Module: crate::parse::fixture
// Provides: {"impl_176"}
// Dependencies: {}
impl Parse for FixtureItem { fn parse (input : ParseStream) -> syn :: Result < Self > { if input . peek2 (Token ! [=]) { input . parse :: < ArgumentValue > () . map (| v | v . into ()) } else { input . parse :: < Fixture > () . map (| v | v . into ()) } } }
};
}
