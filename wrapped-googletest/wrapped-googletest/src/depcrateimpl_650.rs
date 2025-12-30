// Generated macro for impl_650 (impl)
macro_rules! Depcrateimpl_650 {
() => {
// Module: crate
// Provides: {"impl_650"}
// Dependencies: {}
impl < T > GoogleTestSupport for std :: result :: Result < T , TestAssertionFailure > { fn and_log_failure (self) { TestOutcome :: ensure_test_context_present () ; if let Err (failure) = self { failure . log () ; } } fn failure_message (mut self , message : impl Into < String >) -> Self { if let Err (ref mut failure) = self { failure . custom_message = Some (message . into ()) ; } self } fn with_failure_message (mut self , provider : impl FnOnce () -> String) -> Self { if let Err (ref mut failure) = self { failure . custom_message = Some (provider ()) ; } self } }
};
}
