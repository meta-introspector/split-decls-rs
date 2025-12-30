// Generated macro for impl_208 (impl)
macro_rules! Depcrate_sys_timeimpl_208 {
() => {
// Module: crate::sys::time
// Provides: {"impl_208"}
// Dependencies: {}
impl From < TimeSpec > for Duration { fn from (timespec : TimeSpec) -> Self { Duration :: new (timespec . 0 . tv_sec as u64 , timespec . 0 . tv_nsec as u32) } }
};
}
