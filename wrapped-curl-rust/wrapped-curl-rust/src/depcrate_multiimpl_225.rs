// Generated macro for impl_225 (impl)
macro_rules! Depcrate_multiimpl_225 {
() => {
// Module: crate::multi
// Provides: {"impl_225"}
// Dependencies: {}
# [cfg (unix)] impl From < pollfd > for WaitFd { fn from (pfd : pollfd) -> WaitFd { let mut events = 0 ; if pfd . events & POLLIN == POLLIN { events |= curl_sys :: CURL_WAIT_POLLIN ; } if pfd . events & POLLPRI == POLLPRI { events |= curl_sys :: CURL_WAIT_POLLPRI ; } if pfd . events & POLLOUT == POLLOUT { events |= curl_sys :: CURL_WAIT_POLLOUT ; } WaitFd { inner : curl_sys :: curl_waitfd { fd : pfd . fd , events , revents : 0 , } , } } }
};
}
