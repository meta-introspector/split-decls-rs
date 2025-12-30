// Generated macro for impl_108 (impl)
macro_rules! Depcrate_taskimpl_108 {
() => {
// Module: crate::task
// Provides: {"impl_108"}
// Dependencies: {}
impl < W : Wake + Send + Sync + 'static > From < Arc < W > > for RawWaker { # [doc = " Use a `Wake`-able type as a `RawWaker`."] # [doc = ""] # [doc = " No heap allocations or atomic operations are used for this conversion."] fn from (waker : Arc < W >) -> Self { raw_waker (waker) } }
};
}
