// Generated macro for WaitUntil (struct)
macro_rules! Depcrate_future_wait_untilWaitUntil {
() => {
// Module: crate::future::wait_until
// Provides: {"WaitUntil"}
// Dependencies: {}
# [doc = " Suspends a future until the specified deadline."] # [doc = ""] # [doc = " This `struct` is created by the [`wait_until`] method on [`FutureExt`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`wait_until`]: crate::future::FutureExt::wait_until"] # [doc = " [`FutureExt`]: crate::future::FutureExt"] # [derive (Debug)] # [pin_project :: pin_project] # [must_use = "futures do nothing unless polled or .awaited"] pub struct WaitUntil < F , D > { # [pin] future : F , # [pin] deadline : D , state : State , }
};
}
