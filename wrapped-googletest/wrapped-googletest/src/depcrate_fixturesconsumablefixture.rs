// Generated macro for ConsumableFixture (trait)
macro_rules! Depcrate_fixturesConsumableFixture {
() => {
// Module: crate::fixtures
// Provides: {"ConsumableFixture"}
// Dependencies: {}
# [doc = " Interface for structure to be set up before the test case."] # [doc = " Types implementing `ConsumableFixture` can be passed by value to"] # [doc = " a test function."] # [doc = ""] # [doc = " ```ignore"] # [doc = " struct MyFixture { ... }"] # [doc = ""] # [doc = " impl ConsumableFixture for MyFixture { ... }"] # [doc = ""] # [doc = " #[gtest]"] # [doc = " fn test_with_fixture(my_fixture: MyFixture) {...}"] # [doc = " ```"] pub trait ConsumableFixture : Sized { # [doc = " Factory method of the `ConsumableFixture`."] # [doc = ""] # [doc = " This method is called by the test harness before the test case"] # [doc = " that uses this fixture. If this method returns an `Err(...)`,"] # [doc = " then the test case is not evaluated, automatically fails, and"] # [doc = " only the fixtures previously set up are torn down."] fn set_up () -> crate :: Result < Self > ; }
};
}
