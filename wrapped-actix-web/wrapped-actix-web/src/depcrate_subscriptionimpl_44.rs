// Generated macro for impl_44 (impl)
macro_rules! Depcrate_subscriptionimpl_44 {
() => {
// Module: crate::subscription
// Provides: {"impl_44"}
// Dependencies: {}
impl < E , OnInit , OnInitFut , OnPing , OnPingFut > GraphQLSubscriptionActor < E , OnInit , OnPing > where E : Executor , OnInit : FnOnce (serde_json :: Value) -> OnInitFut + Unpin + Send + 'static , OnInitFut : Future < Output = Result < Data > > + Send + 'static , OnPing : FnOnce (Option < & Data > , Option < serde_json :: Value >) -> OnPingFut + Clone + Unpin + Send + 'static , OnPingFut : Future < Output = Result < Option < serde_json :: Value > > > + Send + 'static , { fn send_heartbeats (& self , ctx : & mut WebsocketContext < Self >) { ctx . run_interval (HEARTBEAT_INTERVAL , | act , ctx | { if Instant :: now () . duration_since (act . last_heartbeat) > CLIENT_TIMEOUT { ctx . stop () ; } ctx . ping (b"") ; }) ; } }
};
}
