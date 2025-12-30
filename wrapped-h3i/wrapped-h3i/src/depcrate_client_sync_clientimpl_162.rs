// Generated macro for impl_162 (impl)
macro_rules! Depcrate_client_sync_clientimpl_162 {
() => {
// Module: crate::client::sync_client
// Provides: {"impl_162"}
// Dependencies: {}
impl SyncClient { fn new (close_trigger_frames : Option < CloseTriggerFrames >) -> Self { Self { streams : StreamMap :: new (close_trigger_frames) , .. Default :: default () } } }
};
}
