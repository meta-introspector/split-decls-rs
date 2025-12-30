// Generated macro for impl_11 (impl)
macro_rules! Depcrate_brokerimpl_11 {
() => {
// Module: crate::broker
// Provides: {"impl_11"}
// Dependencies: {}
# [doc = " The system service actor that keeps track of subscriptions and routes messages to them."] impl Broker < SystemBroker > { # [doc = " Send messages asynchronously via the broker. It can be called from with"] # [doc = " actors with a `SyncContext`, or where you don't have access to `self`. e.g. From within"] # [doc = " a `HttpHandler` from `actix-web`."] pub fn issue_async < M : BrokerMsg > (msg : M) { let broker = Self :: from_registry () ; broker . do_send (IssueAsync (msg , TypeId :: of :: < Self > ())) ; } }
};
}
