// Generated macro for poll_watcher_channel (function)
macro_rules! Depcrate_testpoll_watcher_channel {
() => {
// Module: crate::test
// Provides: {"poll_watcher_channel"}
// Dependencies: {}
# [doc = " Creates a [`PollWatcher`] with `with_compare_contents(true)` and manual polling."] # [doc = ""] # [doc = " Returned [`Receiver`] will send a message to poll changes before wait-methods"] pub fn poll_watcher_channel () -> (TestWatcher < PollWatcher > , Receiver) { let (tx , rx) = mpsc :: channel () ; let watcher = PollWatcher :: new (tx , Config :: default () . with_compare_contents (true) . with_manual_polling () ,) . expect ("Unable to create PollWatcher") ; let sender = watcher . poll_sender () ; let watcher = TestWatcher { watcher , kind : PollWatcher :: kind () , } ; let rx = Receiver { rx , timeout : Receiver :: DEFAULT_TIMEOUT , detect_changes : Some (Box :: new (move | | { sender . send (()) . expect ("PollWatcher receiver part was disconnected") })) , kind : watcher . kind , } ; (watcher , rx) }
};
}
