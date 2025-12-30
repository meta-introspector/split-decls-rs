// Generated macro for impl_1188 (impl)
macro_rules! Depcrate_shims_unix_unnamed_socketimpl_1188 {
() => {
// Module: crate::shims::unix::unnamed_socket
// Provides: {"impl_1188"}
// Dependencies: {}
impl UnixFileDescription for AnonSocket { fn get_epoll_ready_events < 'tcx > (& self) -> InterpResult < 'tcx , EpollReadyEvents > { let mut epoll_ready_events = EpollReadyEvents :: new () ; if let Some (readbuf) = & self . readbuf { if ! readbuf . borrow () . buf . is_empty () { epoll_ready_events . epollin = true ; } } else { epoll_ready_events . epollin = true ; } if let Some (peer_fd) = self . peer_fd () . upgrade () { if let Some (writebuf) = & peer_fd . readbuf { let data_size = writebuf . borrow () . buf . len () ; let available_space = MAX_SOCKETPAIR_BUFFER_CAPACITY . strict_sub (data_size) ; if available_space != 0 { epoll_ready_events . epollout = true ; } } else { epoll_ready_events . epollout = true ; } } else { epoll_ready_events . epollrdhup = true ; epoll_ready_events . epollhup = true ; epoll_ready_events . epollin = true ; epoll_ready_events . epollout = true ; if self . peer_lost_data . get () { epoll_ready_events . epollerr = true ; } } interp_ok (epoll_ready_events) } }
};
}
