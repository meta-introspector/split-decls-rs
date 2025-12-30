// Generated macro for ReadDirectoryRequest (struct)
macro_rules! Depcrate_windowsReadDirectoryRequest {
() => {
// Module: crate::windows
// Provides: {"ReadDirectoryRequest"}
// Dependencies: {}
struct ReadDirectoryRequest { event_handler : Arc < Mutex < dyn EventHandler > > , buffer : [u8 ; BUF_SIZE as usize] , handle : HANDLE , data : ReadData , action_tx : Sender < Action > , }
};
}
