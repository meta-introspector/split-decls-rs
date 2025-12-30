// Generated macro for Epoll (struct)
macro_rules! Depcrate_shims_unix_linux_like_epollEpoll {
() => {
// Module: crate::shims::unix::linux_like::epoll
// Provides: {"Epoll"}
// Dependencies: {}
# [doc = " An `Epoll` file descriptor connects file handles and epoll events"] # [derive (Debug , Default)] struct Epoll { # [doc = " A map of EpollEventInterests registered under this epoll instance."] # [doc = " Each entry is differentiated using FdId and file descriptor value."] interest_list : RefCell < BTreeMap < (FdId , i32) , Rc < RefCell < EpollEventInterest > > > > , # [doc = " A map of EpollEventInstance that will be returned when `epoll_wait` is called."] # [doc = " Similar to interest_list, the entry is also differentiated using FdId"] # [doc = " and file descriptor value."] ready_list : ReadyList , # [doc = " A list of thread ids blocked on this epoll instance."] blocked_tid : RefCell < Vec < ThreadId > > , }
};
}
