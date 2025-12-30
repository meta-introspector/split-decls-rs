// Generated macro for impl_81 (impl)
macro_rules! Depcrate_fixturesimpl_81 {
() => {
// Module: crate::fixtures
// Provides: {"impl_81"}
// Dependencies: {}
impl < T : Default > ConsumableFixture for FixtureOf < T > { fn set_up () -> crate :: Result < Self > { Ok (Self (T :: default ())) } }
};
}
