// Generated macro for impl_185 (impl)
macro_rules! Depcrate_future_future_sharedimpl_185 {
() => {
// Module: crate::future::future::shared
// Provides: {"impl_185"}
// Dependencies: {}
impl ArcWake for Notifier { fn wake_by_ref (arc_self : & Arc < Self >) { # [cfg (feature = "std")] let wakers = & mut * arc_self . wakers . lock () . unwrap () ; # [cfg (not (feature = "std"))] let wakers = & mut * arc_self . wakers . lock () ; if let Some (wakers) = wakers . as_mut () { for (_key , opt_waker) in wakers { if let Some (waker) = opt_waker . take () { waker . wake () ; } } } } }
};
}
