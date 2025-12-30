// Generated macro for ready_list_next (function)
macro_rules! Depcrate_shims_unix_linux_like_epollready_list_next {
() => {
// Module: crate::shims::unix::linux_like::epoll
// Provides: {"ready_list_next"}
// Dependencies: {}
# [doc = " This function takes in ready list and returns EpollEventInstance with file description"] # [doc = " that is not closed."] fn ready_list_next (ecx : & MiriInterpCx < '_ > , ready_list : & mut BTreeMap < (FdId , i32) , EpollEventInstance > ,) -> Option < EpollEventInstance > { while let Some ((epoll_key , epoll_event_instance)) = ready_list . pop_first () { if ecx . machine . epoll_interests . get_epoll_interest (epoll_key . 0) . is_some () { return Some (epoll_event_instance) ; } } None }
};
}
