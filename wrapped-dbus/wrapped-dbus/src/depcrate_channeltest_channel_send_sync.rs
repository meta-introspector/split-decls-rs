// Generated macro for test_channel_send_sync (function)
macro_rules! Depcrate_channeltest_channel_send_sync {
() => {
// Module: crate::channel
// Provides: {"test_channel_send_sync"}
// Dependencies: {}
# [test] fn test_channel_send_sync () { fn is_send < T : Send > (_ : & T) { } fn is_sync < T : Sync > (_ : & T) { } let c = Channel :: get_private (BusType :: Session) . unwrap () ; is_send (& c) ; is_sync (& c) ; }
};
}
