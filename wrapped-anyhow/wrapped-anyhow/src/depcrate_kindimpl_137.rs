// Generated macro for impl_137 (impl)
macro_rules! Depcrate_kindimpl_137 {
() => {
// Module: crate::kind
// Provides: {"impl_137"}
// Dependencies: {}
impl Adhoc { # [cold] pub fn new < M > (self , message : M) -> Error where M : Display + Debug + Send + Sync + 'static , { Error :: construct_from_adhoc (message , backtrace ! ()) } }
};
}
