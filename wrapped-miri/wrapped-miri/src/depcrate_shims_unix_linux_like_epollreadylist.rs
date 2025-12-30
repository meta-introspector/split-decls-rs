// Generated macro for ReadyList (struct)
macro_rules! Depcrate_shims_unix_linux_like_epollReadyList {
() => {
// Module: crate::shims::unix::linux_like::epoll
// Provides: {"ReadyList"}
// Dependencies: {}
# [derive (Debug , Default)] struct ReadyList { mapping : RefCell < BTreeMap < (FdId , i32) , EpollEventInstance > > , }
};
}
