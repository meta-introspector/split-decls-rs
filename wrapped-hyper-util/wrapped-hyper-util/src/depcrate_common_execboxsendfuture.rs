// Generated macro for BoxSendFuture (type)
macro_rules! Depcrate_common_execBoxSendFuture {
() => {
// Module: crate::common::exec
// Provides: {"BoxSendFuture"}
// Dependencies: {}
pub (crate) type BoxSendFuture = Pin < Box < dyn Future < Output = () > + Send > > ;
};
}
