// Generated macro for EpollEventInstance (struct)
macro_rules! Depcrate_shims_unix_linux_like_epollEpollEventInstance {
() => {
// Module: crate::shims::unix::linux_like::epoll
// Provides: {"EpollEventInstance"}
// Dependencies: {}
# [doc = " EpollEventInstance contains information that will be returned by epoll_wait."] # [derive (Debug)] pub struct EpollEventInstance { # [doc = " Xor-ed event types that happened to the file description."] events : u32 , # [doc = " Original data retrieved from `epoll_event` during `epoll_ctl`."] data : u64 , # [doc = " The release clock associated with this event."] clock : VClock , }
};
}
