// Generated macro for impl_1109 (impl)
macro_rules! Depcrate_test_runner_errorsimpl_1109 {
() => {
// Module: crate::test_runner::errors
// Provides: {"impl_1109"}
// Dependencies: {}
impl fmt :: Display for TestCaseError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { TestCaseError :: Reject (ref whence) => { write ! (f , "Input rejected at {}" , whence) } TestCaseError :: Fail (ref why) => write ! (f , "Case failed: {}" , why) , } } }
};
}
