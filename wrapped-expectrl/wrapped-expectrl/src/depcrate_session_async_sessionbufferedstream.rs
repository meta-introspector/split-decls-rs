// Generated macro for BufferedStream (struct)
macro_rules! Depcrate_session_async_sessionBufferedStream {
() => {
// Module: crate::session::async_session
// Provides: {"BufferedStream"}
// Dependencies: {}
# [doc = " Session represents a spawned process and its streams."] # [doc = " It controlls process and communication with it."] # [derive (Debug)] struct BufferedStream < S > { stream : S , buffer : Vec < u8 > , length : usize , }
};
}
