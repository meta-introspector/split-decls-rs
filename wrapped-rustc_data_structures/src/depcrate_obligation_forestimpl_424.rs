// Generated macro for impl_424 (impl)
macro_rules! Depcrate_obligation_forestimpl_424 {
() => {
// Module: crate::obligation_forest
// Provides: {"impl_424"}
// Dependencies: {}
impl < O , E > OutcomeTrait for Outcome < O , E > { type Error = Error < O , E > ; type Obligation = O ; fn new () -> Self { Self { errors : vec ! [] } } fn record_completed (& mut self , _outcome : & Self :: Obligation) { } fn record_error (& mut self , error : Self :: Error) { self . errors . push (error) } }
};
}
