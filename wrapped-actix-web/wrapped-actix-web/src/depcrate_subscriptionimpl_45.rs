// Generated macro for impl_45 (impl)
macro_rules! Depcrate_subscriptionimpl_45 {
() => {
// Module: crate::subscription
// Provides: {"impl_45"}
// Dependencies: {}
impl < E , OnInit , OnInitFut , OnPing , OnPingFut > Actor for GraphQLSubscriptionActor < E , OnInit , OnPing > where E : Executor , OnInit : FnOnce (serde_json :: Value) -> OnInitFut + Unpin + Send + 'static , OnInitFut : Future < Output = Result < Data > > + Send + 'static , OnPing : FnOnce (Option < & Data > , Option < serde_json :: Value >) -> OnPingFut + Clone + Unpin + Send + 'static , OnPingFut : Future < Output = Result < Option < serde_json :: Value > > > + Send + 'static , { type Context = WebsocketContext < Self > ; fn started (& mut self , ctx : & mut Self :: Context) { self . send_heartbeats (ctx) ; let (tx , rx) = async_channel :: unbounded () ; WebSocket :: new (self . executor . clone () , rx , self . protocol) . connection_data (self . data . take () . unwrap ()) . on_connection_init (self . on_connection_init . take () . unwrap ()) . on_ping (self . on_ping . clone ()) . keepalive_timeout (self . keepalive_timeout) . into_actor (self) . map (| response , _act , ctx | match response { WsMessage :: Text (text) => ctx . text (text) , WsMessage :: Close (code , msg) => ctx . close (Some (CloseReason { code : code . into () , description : Some (msg) , })) , }) . finish () . spawn (ctx) ; self . messages = Some (tx) ; } }
};
}
