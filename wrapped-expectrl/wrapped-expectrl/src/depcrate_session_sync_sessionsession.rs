// Generated macro for Session (struct)
macro_rules! Depcrate_session_sync_sessionSession {
() => {
// Module: crate::session::sync_session
// Provides: {"Session"}
// Dependencies: {}
# [doc = " Session represents a spawned process and its streams."] # [doc = " It controlls process and communication with it."] # [derive (Debug)] pub struct Session < P , S > { proc : P , stream : TryStream < S > , expect_timeout : Option < Duration > , expect_lazy : bool , }
};
}
