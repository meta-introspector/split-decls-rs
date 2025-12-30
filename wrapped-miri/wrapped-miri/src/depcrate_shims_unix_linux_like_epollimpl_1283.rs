// Generated macro for impl_1283 (impl)
macro_rules! Depcrate_shims_unix_linux_like_epollimpl_1283 {
() => {
// Module: crate::shims::unix::linux_like::epoll
// Provides: {"impl_1283"}
// Dependencies: {}
impl EpollInterestTable { pub (crate) fn new () -> Self { EpollInterestTable (BTreeMap :: new ()) } pub fn insert_epoll_interest (& mut self , id : FdId , fd : Weak < RefCell < EpollEventInterest > >) { match self . 0 . get_mut (& id) { Some (fds) => { fds . push (fd) ; } None => { let vec = vec ! [fd] ; self . 0 . insert (id , vec) ; } } } pub fn get_epoll_interest (& self , id : FdId) -> Option < & Vec < Weak < RefCell < EpollEventInterest > > > > { self . 0 . get (& id) } pub fn get_epoll_interest_mut (& mut self , id : FdId ,) -> Option < & mut Vec < Weak < RefCell < EpollEventInterest > > > > { self . 0 . get_mut (& id) } pub fn remove (& mut self , id : FdId) { self . 0 . remove (& id) ; } }
};
}
