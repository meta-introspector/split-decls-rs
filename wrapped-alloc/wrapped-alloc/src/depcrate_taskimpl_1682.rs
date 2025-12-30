// Generated macro for impl_1682 (impl)
macro_rules! Depcrate_taskimpl_1682 {
() => {
// Module: crate::task
// Provides: {"impl_1682"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] # [stable (feature = "wake_trait" , since = "1.51.0")] impl < W : Wake + Send + Sync + 'static > From < Arc < W > > for Waker { # [doc = " Use a [`Wake`]-able type as a `Waker`."] # [doc = ""] # [doc = " No heap allocations or atomic operations are used for this conversion."] fn from (waker : Arc < W >) -> Waker { unsafe { Waker :: from_raw (raw_waker (waker)) } } }
};
}
