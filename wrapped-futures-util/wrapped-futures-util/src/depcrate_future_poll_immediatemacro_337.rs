// Generated macro for macro_337 (macro)
macro_rules! Depcrate_future_poll_immediatemacro_337 {
() => {
// Module: crate::future::poll_immediate
// Provides: {"macro_337"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`poll_immediate`](poll_immediate()) function."] # [doc = ""] # [doc = " It will never return [Poll::Pending](core::task::Poll::Pending)"] # [derive (Debug , Clone)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct PollImmediate < T > { # [pin] future : Option < T > } }
};
}
