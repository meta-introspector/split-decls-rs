// Generated macro for impl_1687 (impl)
macro_rules! Depcrate_taskimpl_1687 {
() => {
// Module: crate::task
// Provides: {"impl_1687"}
// Dependencies: {}
# [allow (ineffective_unstable_trait_impl)] # [unstable (feature = "local_waker" , issue = "118959")] impl < W : LocalWake + 'static > From < Rc < W > > for RawWaker { # [doc = " Use a `Wake`-able type as a `RawWaker`."] # [doc = ""] # [doc = " No heap allocations or atomic operations are used for this conversion."] fn from (waker : Rc < W >) -> RawWaker { local_raw_waker (waker) } }
};
}
