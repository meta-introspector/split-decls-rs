// Generated macro for send_stream_stopped (function)
macro_rules! Depcrate_send_streamsend_stream_stopped {
() => {
// Module: crate::send_stream
// Provides: {"send_stream_stopped"}
// Dependencies: {}
# [doc = " Check if a send stream is stopped."] # [doc = ""] # [doc = " Returns `Some` if the stream is stopped or the connection is closed."] # [doc = " Returns `None` if the stream is not stopped."] fn send_stream_stopped (conn : & mut State , stream : StreamId , is_0rtt : bool ,) -> Option < Result < Option < VarInt > , StoppedError > > { if is_0rtt && conn . check_0rtt () . is_err () { return Some (Err (StoppedError :: ZeroRttRejected)) ; } match conn . inner . send_stream (stream) . stopped () { Err (ClosedStream { .. }) => Some (Ok (None)) , Ok (Some (error_code)) => Some (Ok (Some (error_code))) , Ok (None) => conn . error . clone () . map (| error | Err (error . into ())) , } }
};
}
