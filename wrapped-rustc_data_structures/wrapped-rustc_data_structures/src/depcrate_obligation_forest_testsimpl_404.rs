// Generated macro for impl_404 (impl)
macro_rules! Depcrate_obligation_forest_testsimpl_404 {
() => {
// Module: crate::obligation_forest::tests
// Provides: {"impl_404"}
// Dependencies: {}
impl < O , E > OutcomeTrait for TestOutcome < O , E > where O : Clone , { type Error = Error < O , E > ; type Obligation = O ; fn new () -> Self { Self { errors : vec ! [] , completed : vec ! [] } } fn record_completed (& mut self , outcome : & Self :: Obligation) { self . completed . push (outcome . clone ()) } fn record_error (& mut self , error : Self :: Error) { self . errors . push (error) } }
};
}
