// Generated macro for impl_110 (impl)
macro_rules! Depcrate_testerimpl_110 {
() => {
// Module: crate::tester
// Provides: {"impl_110"}
// Dependencies: {}
impl < A , E > Testable for Result < A , E > where A : Testable , E : Debug + 'static , { fn result (& self , g : & mut Gen) -> TestResult { match * self { Ok (ref r) => r . result (g) , Err (ref err) => TestResult :: error (format ! ("{err:?}")) , } } }
};
}
