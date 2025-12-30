// Generated macro for impl_1116 (impl)
macro_rules! Depcrate_test_runner_errorsimpl_1116 {
() => {
// Module: crate::test_runner::errors
// Provides: {"impl_1116"}
// Dependencies: {}
impl < T , E > ProptestResultExt < T , E > for Result < T , E > { # [track_caller] fn prop_assume_ok (self) -> Result < T , TestCaseError > where E : fmt :: Debug , { let location = core :: panic :: Location :: caller () ; self . map_err (| err | { TestCaseError :: reject (format ! ("{location}: {err:?}")) }) } }
};
}
