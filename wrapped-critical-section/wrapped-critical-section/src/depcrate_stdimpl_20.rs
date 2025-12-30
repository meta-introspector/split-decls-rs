// Generated macro for impl_20 (impl)
macro_rules! Depcrate_stdimpl_20 {
() => {
// Module: crate::std
// Provides: {"impl_20"}
// Dependencies: {}
unsafe impl crate :: Impl for StdCriticalSection { unsafe fn acquire () -> bool { IS_LOCKED . with (| l | { if l . get () { return true ; } l . set (true) ; let guard = match GLOBAL_MUTEX . lock () { Ok (guard) => guard , Err (err) => { err . into_inner () } } ; GLOBAL_GUARD . write (guard) ; false }) } unsafe fn release (nested_cs : bool) { if ! nested_cs { # [allow (let_underscore_lock)] let _ = GLOBAL_GUARD . assume_init_read () ; IS_LOCKED . with (| l | l . set (false)) ; } } }
};
}
