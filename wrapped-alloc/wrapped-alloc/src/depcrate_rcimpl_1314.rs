// Generated macro for impl_1314 (impl)
macro_rules! Depcrate_rcimpl_1314 {
() => {
// Module: crate::rc
// Provides: {"impl_1314"}
// Dependencies: {}
impl < T > UniqueRc < T > { # [doc = " Creates a new `UniqueRc`."] # [doc = ""] # [doc = " Weak references to this `UniqueRc` can be created with [`UniqueRc::downgrade`]. Upgrading"] # [doc = " these weak references will fail before the `UniqueRc` has been converted into an [`Rc`]."] # [doc = " After converting the `UniqueRc` into an [`Rc`], any weak references created beforehand will"] # [doc = " point to the new [`Rc`]."] # [cfg (not (no_global_oom_handling))] # [unstable (feature = "unique_rc_arc" , issue = "112566")] pub fn new (value : T) -> Self { Self :: new_in (value , Global) } }
};
}
