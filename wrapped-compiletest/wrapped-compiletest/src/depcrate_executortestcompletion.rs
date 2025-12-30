// Generated macro for TestCompletion (struct)
macro_rules! Depcrate_executorTestCompletion {
() => {
// Module: crate::executor
// Provides: {"TestCompletion"}
// Dependencies: {}
# [doc = " Test completion message sent by individual test threads when their test"] # [doc = " finishes (successfully or unsuccessfully)."] struct TestCompletion { id : TestId , outcome : TestOutcome , stdout : Option < Vec < u8 > > , }
};
}
