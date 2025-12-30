// Generated macro for impl_1474 (impl)
macro_rules! Depcrate_shims_unix_linux_like_epollimpl_1474 {
() => {
// Module: crate::shims::unix::linux_like::epoll
// Provides: {"impl_1474"}
// Dependencies: {}
impl EpollEvents { pub fn new () -> Self { EpollEvents { epollin : false , epollout : false , epollrdhup : false , epollhup : false , epollerr : false , } } pub fn get_event_bitmask < 'tcx > (& self , ecx : & MiriInterpCx < 'tcx >) -> u32 { let epollin = ecx . eval_libc_u32 ("EPOLLIN") ; let epollout = ecx . eval_libc_u32 ("EPOLLOUT") ; let epollrdhup = ecx . eval_libc_u32 ("EPOLLRDHUP") ; let epollhup = ecx . eval_libc_u32 ("EPOLLHUP") ; let epollerr = ecx . eval_libc_u32 ("EPOLLERR") ; let mut bitmask = 0 ; if self . epollin { bitmask |= epollin ; } if self . epollout { bitmask |= epollout ; } if self . epollrdhup { bitmask |= epollrdhup ; } if self . epollhup { bitmask |= epollhup ; } if self . epollerr { bitmask |= epollerr ; } bitmask } }
};
}
