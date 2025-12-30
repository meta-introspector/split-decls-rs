// Generated macro for impl_120 (impl)
macro_rules! Depcrate_client_connection_summaryimpl_120 {
() => {
// Module: crate::client::connection_summary
// Provides: {"impl_120"}
// Dependencies: {}
impl < T > From < T > for StreamMap where T : IntoIterator < Item = (u64 , Vec < H3iFrame >) > , { fn from (value : T) -> Self { let stream_frame_map = HashMap :: from_iter (value) ; Self { stream_frame_map , close_trigger_frames : None , } } }
};
}
