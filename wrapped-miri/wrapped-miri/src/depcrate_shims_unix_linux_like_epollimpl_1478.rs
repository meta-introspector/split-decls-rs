// Generated macro for impl_1478 (impl)
macro_rules! Depcrate_shims_unix_linux_like_epollimpl_1478 {
() => {
// Module: crate::shims::unix::linux_like::epoll
// Provides: {"impl_1478"}
// Dependencies: {}
impl EpollInterestTable { pub (crate) fn new () -> Self { EpollInterestTable (BTreeMap :: new ()) } fn insert (& mut self , id : FdId , epoll : & FileDescriptionRef < Epoll >) { let epolls = self . 0 . entry (id) . or_default () ; let idx = epolls . binary_search_by_key (& epoll . id () , | & (id , _) | id) . expect_err ("trying to add an epoll that's already in the list") ; epolls . insert (idx , (epoll . id () , FileDescriptionRef :: downgrade (epoll))) ; } fn remove (& mut self , id : FdId , epoll_id : FdId) { let epolls = self . 0 . entry (id) . or_default () ; let idx = epolls . binary_search_by_key (& epoll_id , | & (id , _) | id) . expect ("trying to remove an epoll that's not in the list") ; epolls . remove (idx) ; } fn get_epolls (& self , id : FdId) -> Option < impl Iterator < Item = & WeakFileDescriptionRef < Epoll > > > { self . 0 . get (& id) . map (| epolls | epolls . iter () . map (| (_id , epoll) | epoll)) } pub fn remove_epolls (& mut self , id : FdId) { if let Some (epolls) = self . 0 . remove (& id) { for epoll in epolls . iter () . filter_map (| (_id , epoll) | epoll . upgrade ()) { epoll . interest_list . borrow_mut () . extract_if (range_for_id (id) , | _ , _ | true) . for_each (drop) ; epoll . ready_set . borrow_mut () . extract_if (range_for_id (id) , | _ | true) . for_each (drop) ; } } } }
};
}
