// Generated macro for impl_951 (impl)
macro_rules! Depcrate_shims_filesimpl_951 {
() => {
// Module: crate::shims::files
// Provides: {"impl_951"}
// Dependencies: {}
impl < T : FileDescription + 'static > FileDescriptionExt for T { fn into_rc_any (self : FileDescriptionRef < Self >) -> Rc < dyn Any > { self . 0 } fn close_ref < 'tcx > (self : FileDescriptionRef < Self > , communicate_allowed : bool , ecx : & mut MiriInterpCx < 'tcx > ,) -> InterpResult < 'tcx , io :: Result < () > > { match Rc :: into_inner (self . 0) { Some (fd) => { ecx . machine . epoll_interests . remove (fd . id) ; fd . inner . close (communicate_allowed , ecx) } None => { interp_ok (Ok (())) } } } }
};
}
