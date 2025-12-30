// Generated macro for BoxSendFuture (type)
macro_rules! Depcrate_client_legacy_clientBoxSendFuture {
() => {
// Module: crate::client::legacy::client
// Provides: {"BoxSendFuture"}
// Dependencies: {}
type BoxSendFuture = Pin < Box < dyn Future < Output = () > + Send > > ;
};
}
