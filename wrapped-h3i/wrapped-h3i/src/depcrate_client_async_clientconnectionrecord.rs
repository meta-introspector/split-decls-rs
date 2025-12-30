// Generated macro for ConnectionRecord (enum)
macro_rules! Depcrate_client_async_clientConnectionRecord {
() => {
// Module: crate::client::async_client
// Provides: {"ConnectionRecord"}
// Dependencies: {}
pub enum ConnectionRecord { StreamedFrame { stream_id : u64 , frame : H3iFrame } , Close (ConnectionCloseDetails) , PathStats (Vec < PathStats >) , ConnectionStats (Stats) , }
};
}
