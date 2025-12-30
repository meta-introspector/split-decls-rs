// Generated macro for EpollInterestTable (struct)
macro_rules! Depcrate_shims_unix_linux_like_epollEpollInterestTable {
() => {
// Module: crate::shims::unix::linux_like::epoll
// Provides: {"EpollInterestTable"}
// Dependencies: {}
# [doc = " The table of all EpollEventInterest."] # [doc = " The BTreeMap key is the FdId of an active file description registered with"] # [doc = " any epoll instance. The value is a list of EpollEventInterest associated"] # [doc = " with that file description."] pub struct EpollInterestTable (BTreeMap < FdId , Vec < Weak < RefCell < EpollEventInterest > > > >) ;
};
}
