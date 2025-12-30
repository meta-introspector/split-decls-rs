// Generated macro for impl_1686 (impl)
macro_rules! Depcrate_taskimpl_1686 {
() => {
// Module: crate::task
// Provides: {"impl_1686"}
// Dependencies: {}
# [unstable (feature = "local_waker" , issue = "118959")] impl < W : LocalWake + 'static > From < Rc < W > > for LocalWaker { # [doc = " Use a `Wake`-able type as a `LocalWaker`."] # [doc = ""] # [doc = " No heap allocations or atomic operations are used for this conversion."] fn from (waker : Rc < W >) -> LocalWaker { unsafe { LocalWaker :: from_raw (local_raw_waker (waker)) } } }
};
}
