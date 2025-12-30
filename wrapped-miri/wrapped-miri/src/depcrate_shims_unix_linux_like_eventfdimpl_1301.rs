// Generated macro for impl_1301 (impl)
macro_rules! Depcrate_shims_unix_linux_like_eventfdimpl_1301 {
() => {
// Module: crate::shims::unix::linux_like::eventfd
// Provides: {"impl_1301"}
// Dependencies: {}
impl UnixFileDescription for EventFd { fn get_epoll_ready_events < 'tcx > (& self) -> InterpResult < 'tcx , EpollReadyEvents > { interp_ok (EpollReadyEvents { epollin : self . counter . get () != 0 , epollout : self . counter . get () != MAX_COUNTER , .. EpollReadyEvents :: new () }) } }
};
}
