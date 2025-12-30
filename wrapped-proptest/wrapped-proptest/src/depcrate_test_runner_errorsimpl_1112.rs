// Generated macro for impl_1112 (impl)
macro_rules! Depcrate_test_runner_errorsimpl_1112 {
() => {
// Module: crate::test_runner::errors
// Provides: {"impl_1112"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Display for TestError < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { TestError :: Abort (ref why) => write ! (f , "Test aborted: {}" , why) , TestError :: Fail (ref why , ref what) => { writeln ! (f , "Test failed: {}." , why) ? ; write ! (f , "minimal failing input: {:#?}" , what) } } } }
};
}
