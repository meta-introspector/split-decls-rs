// Generated macro for Fixture (trait)
macro_rules! Depcrate_fixturesFixture {
() => {
// Module: crate::fixtures
// Provides: {"Fixture"}
// Dependencies: {}
# [doc = " Interface for structure to be set up and torn down as part of a test."] # [doc = " Types implementing `Fixture` can be passed as a reference argument to a"] # [doc = " test function."] # [doc = ""] # [doc = " ```ignore"] # [doc = " struct MyFixture { ... }"] # [doc = ""] # [doc = " impl Fixture for MyFixture { ... }"] # [doc = ""] # [doc = " #[gtest]"] # [doc = " fn test_with_fixture(my_fixture: &MyFixture) {...}"] # [doc = " ```"] pub trait Fixture : Sized { # [doc = " Factory method of the `Fixture`."] # [doc = ""] # [doc = " This method is called by the test harness before the test case"] # [doc = " that uses this fixture. If this method returns an `Err(...)`,"] # [doc = " then the test case is not evaluated, automatically fails, and"] # [doc = " only the fixtures previously set up are torn down."] fn set_up () -> crate :: Result < Self > ; # [doc = " Clean up method for the fixture."] # [doc = ""] # [doc = " This method is called by the test harness after the test case"] # [doc = " that uses this fixture. If the `Fixture` has been set up, the"] # [doc = " test harness will call this method, even if the test case failed"] # [doc = " or panicked."] fn tear_down (self) -> crate :: Result < () > ; }
};
}
