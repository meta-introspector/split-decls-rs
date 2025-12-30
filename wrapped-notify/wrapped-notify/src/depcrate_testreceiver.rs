// Generated macro for Receiver (struct)
macro_rules! Depcrate_testReceiver {
() => {
// Module: crate::test
// Provides: {"Receiver"}
// Dependencies: {}
# [doc = " Waits any events from the watcher and provides with some helper methods"] pub struct Receiver { pub rx : mpsc :: Receiver < Result < Event , Error > > , pub timeout : Duration , pub detect_changes : Option < Box < dyn Fn () > > , pub kind : WatcherKind , }
};
}
