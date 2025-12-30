// Generated macro for Stream (struct)
macro_rules! Depcrate_session_async_sessionStream {
() => {
// Module: crate::session::async_session
// Provides: {"Stream"}
// Dependencies: {}
# [doc = " Session represents a spawned process and its streams."] # [doc = " It controlls process and communication with it."] # [derive (Debug)] struct Stream < S > { stream : BufferedStream < S > , expect_timeout : Option < Duration > , expect_lazy : bool , }
};
}
