// Generated macro for impl_1113 (impl)
macro_rules! Depcrate_test_runner_errorsimpl_1113 {
() => {
// Module: crate::test_runner::errors
// Provides: {"impl_1113"}
// Dependencies: {}
# [cfg (feature = "std")] # [allow (deprecated)] impl < T : fmt :: Debug > :: std :: error :: Error for TestError < T > { fn description (& self) -> & str { match * self { TestError :: Abort (..) => "Abort" , TestError :: Fail (..) => "Fail" , } } }
};
}
