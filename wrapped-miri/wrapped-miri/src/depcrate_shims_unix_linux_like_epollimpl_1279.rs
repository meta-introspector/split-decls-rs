// Generated macro for impl_1279 (impl)
macro_rules! Depcrate_shims_unix_linux_like_epollimpl_1279 {
() => {
// Module: crate::shims::unix::linux_like::epoll
// Provides: {"impl_1279"}
// Dependencies: {}
impl EpollReadyEvents { pub fn new () -> Self { EpollReadyEvents { epollin : false , epollout : false , epollrdhup : false , epollhup : false , epollerr : false , } } pub fn get_event_bitmask < 'tcx > (& self , ecx : & MiriInterpCx < 'tcx >) -> u32 { let epollin = ecx . eval_libc_u32 ("EPOLLIN") ; let epollout = ecx . eval_libc_u32 ("EPOLLOUT") ; let epollrdhup = ecx . eval_libc_u32 ("EPOLLRDHUP") ; let epollhup = ecx . eval_libc_u32 ("EPOLLHUP") ; let epollerr = ecx . eval_libc_u32 ("EPOLLERR") ; let mut bitmask = 0 ; if self . epollin { bitmask |= epollin ; } if self . epollout { bitmask |= epollout ; } if self . epollrdhup { bitmask |= epollrdhup ; } if self . epollhup { bitmask |= epollhup ; } if self . epollerr { bitmask |= epollerr ; } bitmask } }
};
}
