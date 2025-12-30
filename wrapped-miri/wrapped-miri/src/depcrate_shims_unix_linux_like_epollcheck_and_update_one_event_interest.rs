// Generated macro for check_and_update_one_event_interest (function)
macro_rules! Depcrate_shims_unix_linux_like_epollcheck_and_update_one_event_interest {
() => {
// Module: crate::shims::unix::linux_like::epoll
// Provides: {"check_and_update_one_event_interest"}
// Dependencies: {}
# [doc = " This helper function checks whether an epoll notification should be triggered for a specific"] # [doc = " epoll_interest and, if necessary, triggers the notification, and returns whether the"] # [doc = " notification was added/updated. Unlike check_and_update_readiness, this function sends a"] # [doc = " notification to only one epoll instance."] fn check_and_update_one_event_interest < 'tcx > (fd_ref : & DynFileDescriptionRef , interest : & RefCell < EpollEventInterest > , id : FdId , ecx : & MiriInterpCx < 'tcx > ,) -> InterpResult < 'tcx , bool > { let ready_events_bitmask = fd_ref . as_unix (ecx) . get_epoll_ready_events () ? . get_event_bitmask (ecx) ; let epoll_event_interest = interest . borrow () ; let epfd = epoll_event_interest . weak_epfd . upgrade () . unwrap () ; let flags = epoll_event_interest . events & ready_events_bitmask ; if flags != 0 { let epoll_key = (id , epoll_event_interest . fd_num) ; let mut ready_list = epfd . ready_list . mapping . borrow_mut () ; let mut event_instance = EpollEventInstance :: new (flags , epoll_event_interest . data) ; ecx . release_clock (| clock | { event_instance . clock . clone_from (clock) ; }) ; ready_list . insert (epoll_key , event_instance) ; interp_ok (true) } else { interp_ok (false) } }
};
}
