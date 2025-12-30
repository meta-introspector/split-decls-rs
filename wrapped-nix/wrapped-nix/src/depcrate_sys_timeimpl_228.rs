// Generated macro for impl_228 (impl)
macro_rules! Depcrate_sys_timeimpl_228 {
() => {
// Module: crate::sys::time
// Provides: {"impl_228"}
// Dependencies: {}
impl Ord for TimeVal { fn cmp (& self , other : & TimeVal) -> cmp :: Ordering { if self . tv_sec () == other . tv_sec () { self . tv_usec () . cmp (& other . tv_usec ()) } else { self . tv_sec () . cmp (& other . tv_sec ()) } } }
};
}
