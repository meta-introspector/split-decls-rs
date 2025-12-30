// Generated macro for impl_107 (impl)
macro_rules! Depcrate_taskimpl_107 {
() => {
// Module: crate::task
// Provides: {"impl_107"}
// Dependencies: {}
impl < W : Wake + Send + Sync + 'static > From < Arc < W > > for Waker { # [doc = " Use a [`Wake`]-able type as a `Waker`."] # [doc = ""] # [doc = " No heap allocations or atomic operations are used for this conversion."] fn from (waker : Arc < W >) -> Self { unsafe { Self :: from_raw (raw_waker (waker)) } } }
};
}
