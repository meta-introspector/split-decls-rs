// Generated macro for FileDescriptionExt (trait)
macro_rules! Depcrate_shims_filesFileDescriptionExt {
() => {
// Module: crate::shims::files
// Provides: {"FileDescriptionExt"}
// Dependencies: {}
# [doc = " A helper trait to indirectly allow downcasting on `Rc<FdIdWith<dyn _>>`."] # [doc = " Ideally we'd just add a `FdIdWith<Self>: Any` bound to the `FileDescription` trait,"] # [doc = " but that does not allow upcasting."] pub trait FileDescriptionExt : 'static { fn into_rc_any (self : FileDescriptionRef < Self >) -> Rc < dyn Any > ; # [doc = " We wrap the regular `close` function generically, so both handle `Rc::into_inner`"] # [doc = " and epoll interest management."] fn close_ref < 'tcx > (self : FileDescriptionRef < Self > , communicate_allowed : bool , ecx : & mut MiriInterpCx < 'tcx > ,) -> InterpResult < 'tcx , io :: Result < () > > ; }
};
}
