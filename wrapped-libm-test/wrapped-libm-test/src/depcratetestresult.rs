// Generated macro for TestResult (type)
macro_rules! DepcrateTestResult {
() => {
// Module: crate
// Provides: {"TestResult"}
// Dependencies: {}
# [doc = " Result type for tests is usually from `anyhow`. Most times there is no success value to"] # [doc = " propagate."] pub type TestResult < T = () , E = anyhow :: Error > = Result < T , E > ;
};
}
