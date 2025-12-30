// Generated macro for impl_282 (impl)
macro_rules! Depcrate_session_sync_sessionimpl_282 {
() => {
// Module: crate::session::sync_session
// Provides: {"impl_282"}
// Dependencies: {}
# [cfg (unix)] impl < P , S > std :: os :: fd :: AsRawFd for & mut Session < P , S > where S : std :: os :: fd :: AsRawFd , { fn as_raw_fd (& self) -> std :: os :: unix :: prelude :: RawFd { self . get_stream () . as_raw_fd () } }
};
}
