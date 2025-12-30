// Generated macro for Action (enum)
macro_rules! Depcrate_actions_h3Action {
() => {
// Module: crate::actions::h3
// Provides: {"Action"}
// Dependencies: {}
# [doc = " An action which the HTTP/3 client should take."] # [doc = ""] # [doc = " The client iterates over a vector of said actions, executing each one"] # [doc = " sequentially. Note that packets will be flushed when said iteration has"] # [doc = " completed, regardless of if an [`Action::FlushPackets`] was the terminal"] # [doc = " action."] # [derive (Clone , Debug , PartialEq , Eq)] pub enum Action { # [doc = " Send a [quiche::h3::frame::Frame] over a stream."] SendFrame { stream_id : u64 , fin_stream : bool , frame : Frame , } , # [doc = " Send a HEADERS frame over a stream."] SendHeadersFrame { stream_id : u64 , fin_stream : bool , literal_headers : bool , headers : Vec < Header > , frame : Frame , } , # [doc = " Send arbitrary bytes over a stream."] StreamBytes { stream_id : u64 , fin_stream : bool , bytes : Vec < u8 > , } , # [doc = " Send a DATAGRAM frame."] SendDatagram { payload : Vec < u8 > , } , # [doc = " Open a new unidirectional stream."] OpenUniStream { stream_id : u64 , fin_stream : bool , stream_type : u64 , } , # [doc = " Send a RESET_STREAM frame with the given error code."] ResetStream { stream_id : u64 , error_code : u64 , } , # [doc = " Send a STOP_SENDING frame with the given error code."] StopSending { stream_id : u64 , error_code : u64 , } , # [doc = " Send a CONNECTION_CLOSE frame with the given [`ConnectionError`]."] ConnectionClose { error : ConnectionError , } , FlushPackets , # [doc = " Wait for an event. See [WaitType] for the events."] Wait { wait_type : WaitType , } , }
};
}
