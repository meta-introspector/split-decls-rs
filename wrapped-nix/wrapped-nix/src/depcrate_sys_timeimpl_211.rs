// Generated macro for impl_211 (impl)
macro_rules! Depcrate_sys_timeimpl_211 {
() => {
// Module: crate::sys::time
// Provides: {"impl_211"}
// Dependencies: {}
impl Ord for TimeSpec { fn cmp (& self , other : & TimeSpec) -> cmp :: Ordering { if self . tv_sec () == other . tv_sec () { self . tv_nsec () . cmp (& other . tv_nsec ()) } else { self . tv_sec () . cmp (& other . tv_sec ()) } } }
};
}
