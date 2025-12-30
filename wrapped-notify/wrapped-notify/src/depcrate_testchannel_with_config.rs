// Generated macro for channel_with_config (function)
macro_rules! Depcrate_testchannel_with_config {
() => {
// Module: crate::test
// Provides: {"channel_with_config"}
// Dependencies: {}
# [doc = " Creates a [`TestWatcher`] and connected [`Receiver`]"] pub fn channel_with_config < W : Watcher > (config : ChannelConfig) -> (TestWatcher < W > , Receiver) { let (tx , rx) = mpsc :: channel () ; let watcher = W :: new (tx , config . watcher_config) . expect ("Unable to create a watcher") ; (TestWatcher { watcher , kind : W :: kind () , } , Receiver { rx , timeout : config . timeout , detect_changes : None , kind : W :: kind () , } ,) }
};
}
