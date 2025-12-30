// Generated macro for impl_186 (impl)
macro_rules! Depcrate_future_future_sharedimpl_186 {
() => {
// Module: crate::future::future::shared
// Provides: {"impl_186"}
// Dependencies: {}
impl < Fut : Future > WeakShared < Fut > { # [doc = " Attempts to upgrade this [`WeakShared`] into a [`Shared`]."] # [doc = ""] # [doc = " Returns [`None`] if all clones of the [`Shared`] have been dropped or polled"] # [doc = " to completion."] pub fn upgrade (& self) -> Option < Shared < Fut > > { Some (Shared { inner : Some (self . 0 . upgrade () ?) , waker_key : NULL_WAKER_KEY }) } }
};
}
