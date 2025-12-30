// Generated macro for StaticFixture (trait)
macro_rules! Depcrate_fixturesStaticFixture {
() => {
// Module: crate::fixtures
// Provides: {"StaticFixture"}
// Dependencies: {}
# [doc = " Interface for structure to be set up only once before all tests."] # [doc = " Types implementing `StaticFixture` can be passed as a double referenced"] # [doc = " argument to a test function."] # [doc = ""] # [doc = " ```ignore"] # [doc = " struct MyFixture{ ... }"] # [doc = ""] # [doc = " impl StaticFixture for MyFixture { ... }"] # [doc = ""] # [doc = " #[gtest]"] # [doc = " fn test_with_fixture(my_fixture: &&MyFixture){...}"] # [doc = " ```"] pub trait StaticFixture : Sized + Sync + Send { # [doc = " Factory method of the `StaticFixture`."] # [doc = ""] # [doc = " This method is called by the test harness before the first test case"] # [doc = " using this fixture. If this method returns an `Err(...)`, then every"] # [doc = " test case using this fixture is not evaluated and automatically fails."] fn set_up_once () -> crate :: Result < Self > ; }
};
}
