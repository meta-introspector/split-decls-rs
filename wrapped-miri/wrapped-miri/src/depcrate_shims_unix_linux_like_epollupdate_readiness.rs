// Generated macro for update_readiness (function)
macro_rules! Depcrate_shims_unix_linux_like_epollupdate_readiness {
() => {
// Module: crate::shims::unix::linux_like::epoll
// Provides: {"update_readiness"}
// Dependencies: {}
# [doc = " Call this when the interests denoted by `for_each_interest` have their active event set changed"] # [doc = " to `active_events`. The list is provided indirectly via the `for_each_interest` closure, which"] # [doc = " will call its argument closure for each relevant interest."] # [doc = ""] # [doc = " Any `RefCell` should be released by the time `for_each_interest` returns since we will then"] # [doc = " be waking up threads which might require access to those `RefCell`."] fn update_readiness < 'tcx > (ecx : & mut MiriInterpCx < 'tcx > , epoll : & Epoll , active_events : u32 , force_edge : bool , for_each_interest : impl FnOnce (& mut dyn FnMut (EpollEventKey , & mut EpollEventInterest) -> InterpResult < 'tcx > ,) -> InterpResult < 'tcx > ,) -> InterpResult < 'tcx > { let mut ready_set = epoll . ready_set . borrow_mut () ; for_each_interest (& mut | key , interest | { let new_readiness = interest . relevant_events & active_events ; let prev_readiness = std :: mem :: replace (& mut interest . active_events , new_readiness) ; if new_readiness == 0 { ready_set . remove (& key) ; } else if force_edge || new_readiness != prev_readiness & new_readiness { ready_set . insert (key) ; ecx . release_clock (| clock | { interest . clock . join (clock) ; }) ? ; } interp_ok (()) }) ? ; while ! ready_set . is_empty () && let Some (thread_id) = epoll . queue . borrow_mut () . pop_front () { drop (ready_set) ; ecx . unblock_thread (thread_id , BlockReason :: Epoll) ? ; ready_set = epoll . ready_set . borrow_mut () ; } interp_ok (()) }
};
}
