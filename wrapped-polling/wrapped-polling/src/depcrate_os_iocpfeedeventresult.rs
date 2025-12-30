// Generated macro for FeedEventResult (enum)
macro_rules! Depcrate_os_iocpFeedEventResult {
() => {
// Module: crate::os::iocp
// Provides: {"FeedEventResult"}
// Dependencies: {}
# [doc = " The result of calling `feed_event`."] # [derive (Debug)] enum FeedEventResult { # [doc = " No event was yielded."] NoEvent , # [doc = " An event was yielded."] Event (Event) , # [doc = " The poller has been notified."] Notified , }
};
}
