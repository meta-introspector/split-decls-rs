// Generated macro for Shared (struct)
macro_rules! Depcrate_future_future_sharedShared {
() => {
// Module: crate::future::future::shared
// Provides: {"Shared"}
// Dependencies: {}
# [doc = " Future for the [`shared`](super::FutureExt::shared) method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Shared < Fut : Future > { inner : Option < Arc < Inner < Fut > > > , waker_key : usize , }
};
}
