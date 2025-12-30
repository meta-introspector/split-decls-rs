// Generated macro for impl_1475 (impl)
macro_rules! Depcrate_shims_unix_linux_like_epollimpl_1475 {
() => {
// Module: crate::shims::unix::linux_like::epoll
// Provides: {"impl_1475"}
// Dependencies: {}
impl FileDescription for Epoll { fn name (& self) -> & 'static str { "epoll" } fn destroy < 'tcx > (mut self , self_id : FdId , _communicate_allowed : bool , ecx : & mut MiriInterpCx < 'tcx > ,) -> InterpResult < 'tcx , io :: Result < () > > { let mut ids = self . interest_list . get_mut () . keys () . map (| (id , _num) | * id) . collect :: < Vec < _ > > () ; ids . dedup () ; for id in ids { ecx . machine . epoll_interests . remove (id , self_id) ; } interp_ok (Ok (())) } fn as_unix < 'tcx > (& self , _ecx : & MiriInterpCx < 'tcx >) -> & dyn UnixFileDescription { self } }
};
}
