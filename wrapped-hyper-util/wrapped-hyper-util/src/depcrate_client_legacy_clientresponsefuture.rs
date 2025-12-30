// Generated macro for ResponseFuture (struct)
macro_rules! Depcrate_client_legacy_clientResponseFuture {
() => {
// Module: crate::client::legacy::client
// Provides: {"ResponseFuture"}
// Dependencies: {}
# [doc = " A `Future` that will resolve to an HTTP Response."] # [doc = ""] # [doc = " This is returned by `Client::request` (and `Client::get`)."] # [must_use = "futures do nothing unless polled"] pub struct ResponseFuture { inner : SyncWrapper < Pin < Box < dyn Future < Output = Result < Response < hyper :: body :: Incoming > , Error > > + Send > > , > , }
};
}
