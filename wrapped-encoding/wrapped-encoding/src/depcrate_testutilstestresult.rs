// Generated macro for TestResult (struct)
macro_rules! Depcrate_testutilsTestResult {
() => {
// Module: crate::testutils
// Provides: {"TestResult"}
// Dependencies: {}
pub struct TestResult < 'a , Output : 'a + ? Sized + ToOwned > { pub expected_return : (usize , Option < isize >) , pub expected_push : & 'a Output , pub actual_return : (usize , Option < isize >) , pub actual_push : Output :: Owned , }
};
}
