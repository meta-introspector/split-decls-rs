// Generated macro for impl_1666 (impl)
macro_rules! Depcrate_syncimpl_1666 {
() => {
// Module: crate::sync
// Provides: {"impl_1666"}
// Dependencies: {}
impl < T > UniqueArc < T , Global > { # [doc = " Creates a new `UniqueArc`."] # [doc = ""] # [doc = " Weak references to this `UniqueArc` can be created with [`UniqueArc::downgrade`]. Upgrading"] # [doc = " these weak references will fail before the `UniqueArc` has been converted into an [`Arc`]."] # [doc = " After converting the `UniqueArc` into an [`Arc`], any weak references created beforehand will"] # [doc = " point to the new [`Arc`]."] # [cfg (not (no_global_oom_handling))] # [unstable (feature = "unique_rc_arc" , issue = "112566")] # [must_use] pub fn new (value : T) -> Self { Self :: new_in (value , Global) } }
};
}
