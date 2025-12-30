// Generated macro for H3iDriver (struct)
macro_rules! Depcrate_client_async_clientH3iDriver {
() => {
// Module: crate::client::async_client
// Provides: {"H3iDriver"}
// Dependencies: {}
pub struct H3iDriver { buffer : Pooled < ConsumeBuffer > , actions : Vec < Action > , actions_executed : usize , next_fire_time : Instant , waiting_for_responses : WaitingFor , record_tx : mpsc :: UnboundedSender < ConnectionRecord > , stream_parsers : StreamParserMap , close_trigger_seen_rx : oneshot :: Receiver < () > , }
};
}
