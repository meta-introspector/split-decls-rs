// Generated macro for impl_1495 (impl)
macro_rules! Depcrate_shims_unix_linux_like_eventfdimpl_1495 {
() => {
// Module: crate::shims::unix::linux_like::eventfd
// Provides: {"impl_1495"}
// Dependencies: {}
impl UnixFileDescription for EventFd { fn epoll_active_events < 'tcx > (& self) -> InterpResult < 'tcx , EpollEvents > { interp_ok (EpollEvents { epollin : self . counter . get () != 0 , epollout : self . counter . get () != MAX_COUNTER , .. EpollEvents :: new () }) } }
};
}
