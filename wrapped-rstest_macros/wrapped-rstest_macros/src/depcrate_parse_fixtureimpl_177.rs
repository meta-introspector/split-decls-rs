// Generated macro for impl_177 (impl)
macro_rules! Depcrate_parse_fixtureimpl_177 {
() => {
// Module: crate::parse::fixture
// Provides: {"impl_177"}
// Dependencies: {}
impl RefPat for FixtureItem { fn pat (& self) -> & Pat { match self { FixtureItem :: Fixture (ref fix) => & fix . arg , FixtureItem :: ArgumentValue (ref av) => & av . arg , } } }
};
}
