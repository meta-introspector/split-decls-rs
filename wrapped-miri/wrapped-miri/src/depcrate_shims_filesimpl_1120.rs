// Generated macro for impl_1120 (impl)
macro_rules! Depcrate_shims_filesimpl_1120 {
() => {
// Module: crate::shims::files
// Provides: {"impl_1120"}
// Dependencies: {}
impl < T : FileDescription + 'static > FileDescriptionExt for T { fn into_rc_any (self : FileDescriptionRef < Self >) -> Rc < dyn Any > { self . 0 } fn close_ref < 'tcx > (self : FileDescriptionRef < Self > , communicate_allowed : bool , ecx : & mut MiriInterpCx < 'tcx > ,) -> InterpResult < 'tcx , io :: Result < () > > { match Rc :: into_inner (self . 0) { Some (fd) => { ecx . machine . epoll_interests . remove_epolls (fd . id) ; fd . inner . destroy (fd . id , communicate_allowed , ecx) } None => { interp_ok (Ok (())) } } } }
};
}
