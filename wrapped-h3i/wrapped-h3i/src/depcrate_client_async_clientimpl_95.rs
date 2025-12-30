// Generated macro for impl_95 (impl)
macro_rules! Depcrate_client_async_clientimpl_95 {
() => {
// Module: crate::client::async_client
// Provides: {"impl_95"}
// Dependencies: {}
impl Future for BuildingConnectionSummary { type Output = ConnectionSummary ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Self :: Output > { while let Poll :: Ready (Some (record)) = self . rx . poll_recv (cx) { let summary = self . summary . as_mut () . expect ("summary already taken") ; match record { ConnectionRecord :: StreamedFrame { stream_id , frame } => { let stream_map = & mut summary . stream_map ; stream_map . insert (stream_id , frame) ; if stream_map . all_close_trigger_frames_seen () { if let Some (expected_tx) = self . seen_all_close_trigger_frames . take () { let _ = expected_tx . send (()) ; } } } , ConnectionRecord :: ConnectionStats (s) => summary . stats = Some (s) , ConnectionRecord :: PathStats (ps) => summary . path_stats = ps , ConnectionRecord :: Close (d) => summary . conn_close_details = d , } ; } if self . rx . is_closed () { let summary = self . summary . take () . expect ("summary already taken") ; Poll :: Ready (summary) } else { Poll :: Pending } } }
};
}
