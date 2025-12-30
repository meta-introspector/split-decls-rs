// Generated macro for Poller (struct)
macro_rules! Depcrate_os_kqueuePoller {
() => {
// Module: crate::os::kqueue
// Provides: {"Poller"}
// Dependencies: {}
# [doc = " Interface to kqueue."] # [derive (Debug)] pub struct Poller { # [doc = " File descriptor for the kqueue instance."] kqueue_fd : OwnedFd , # [doc = " List of sources currently registered in this poller."] # [doc = ""] # [doc = " This is used to make sure the same source is not registered twice."] sources : RwLock < HashSet < SourceId > > , # [doc = " Notification pipe for waking up the poller."] # [doc = ""] # [doc = " On platforms that support `EVFILT_USER`, this uses that to wake up the poller. Otherwise, it"] # [doc = " uses a pipe."] notify : notify :: Notify , }
};
}
