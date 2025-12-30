// Generated macro for channel (function)
macro_rules! Depcrate_testchannel {
() => {
// Module: crate::test
// Provides: {"channel"}
// Dependencies: {}
# [doc = " Creates a [`TestWatcher`] and connected [`Receiver`]"] pub fn channel < W : Watcher > () -> (TestWatcher < W > , Receiver) { channel_with_config (Default :: default ()) }
};
}
