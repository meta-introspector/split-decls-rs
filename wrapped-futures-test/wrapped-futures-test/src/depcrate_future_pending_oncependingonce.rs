// Generated macro for PendingOnce (struct)
macro_rules! Depcrate_future_pending_oncePendingOnce {
() => {
// Module: crate::future::pending_once
// Provides: {"PendingOnce"}
// Dependencies: {}
# [doc = " Combinator that guarantees one [`Poll::Pending`] before polling its inner"] # [doc = " future."] # [doc = ""] # [doc = " This is created by the"] # [doc = " [`FutureTestExt::pending_once`](super::FutureTestExt::pending_once)"] # [doc = " method."] # [pin_project] # [derive (Debug , Clone)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct PendingOnce < Fut > { # [pin] future : Fut , polled_before : bool , }
};
}
