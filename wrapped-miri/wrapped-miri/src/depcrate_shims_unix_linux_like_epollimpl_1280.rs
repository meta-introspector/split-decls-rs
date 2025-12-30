// Generated macro for impl_1280 (impl)
macro_rules! Depcrate_shims_unix_linux_like_epollimpl_1280 {
() => {
// Module: crate::shims::unix::linux_like::epoll
// Provides: {"impl_1280"}
// Dependencies: {}
impl FileDescription for Epoll { fn name (& self) -> & 'static str { "epoll" } fn close < 'tcx > (self , _communicate_allowed : bool , _ecx : & mut MiriInterpCx < 'tcx > ,) -> InterpResult < 'tcx , io :: Result < () > > { interp_ok (Ok (())) } fn as_unix < 'tcx > (& self , _ecx : & MiriInterpCx < 'tcx >) -> & dyn UnixFileDescription { self } }
};
}
