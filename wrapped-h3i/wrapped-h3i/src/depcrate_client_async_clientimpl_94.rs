// Generated macro for impl_94 (impl)
macro_rules! Depcrate_client_async_clientimpl_94 {
() => {
// Module: crate::client::async_client
// Provides: {"impl_94"}
// Dependencies: {}
impl BuildingConnectionSummary { fn new (rx : mpsc :: UnboundedReceiver < ConnectionRecord > , close_trigger_frames : Option < CloseTriggerFrames > , trigger_frame_tx : oneshot :: Sender < () > ,) -> Self { let summary = ConnectionSummary { stream_map : StreamMap :: new (close_trigger_frames) , .. Default :: default () } ; Self { rx , summary : Some (summary) , seen_all_close_trigger_frames : Some (trigger_frame_tx) , } } }
};
}
