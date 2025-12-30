// Generated macro for impl_1683 (impl)
macro_rules! Depcrate_taskimpl_1683 {
() => {
// Module: crate::task
// Provides: {"impl_1683"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] # [stable (feature = "wake_trait" , since = "1.51.0")] impl < W : Wake + Send + Sync + 'static > From < Arc < W > > for RawWaker { # [doc = " Use a `Wake`-able type as a `RawWaker`."] # [doc = ""] # [doc = " No heap allocations or atomic operations are used for this conversion."] fn from (waker : Arc < W >) -> RawWaker { raw_waker (waker) } }
};
}
