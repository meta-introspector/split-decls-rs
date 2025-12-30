// Generated macro for impl_123 (impl)
macro_rules! Depcrate_client_connection_summaryimpl_123 {
() => {
// Module: crate::client::connection_summary
// Provides: {"impl_123"}
// Dependencies: {}
impl CloseTriggerFrames { # [doc = " Create a new [`CloseTriggerFrames`]. If all expected frames are"] # [doc = " received, h3i will close the connection with an application-level"] # [doc = " CONNECTION_CLOSE frame with error code 0x100."] pub fn new (frames : Vec < CloseTriggerFrame >) -> Self { Self :: new_with_connection_close (frames , ConnectionError { is_app : true , error_code : quiche :: h3 :: WireErrorCode :: NoError as u64 , reason : b"saw all close trigger frames" . to_vec () , }) } # [doc = " Create a new [`CloseTriggerFrames`] with a custom close frame. When all"] # [doc = " close trigger frames are received, h3i will close the connection with"] # [doc = " the level, error code, and reason from `close_with`."] pub fn new_with_connection_close (frames : Vec < CloseTriggerFrame > , close_with : ConnectionError ,) -> Self { Self { missing : frames , close_with , } } fn receive_frame (& mut self , stream_id : u64 , frame : & H3iFrame) { for (i , trigger) in self . missing . iter_mut () . enumerate () { if trigger . is_equivalent (frame) && trigger . stream_id () == stream_id { self . missing . remove (i) ; break ; } } } fn saw_all_trigger_frames (& self) -> bool { self . missing . is_empty () } fn missing_triggers (& self) -> Vec < CloseTriggerFrame > { self . missing . clone () } }
};
}
