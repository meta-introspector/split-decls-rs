// Generated macro for macro_323 (macro)
macro_rules! Depcrate_fut_future_timeoutmacro_323 {
() => {
// Module: crate::fut::future::timeout
// Provides: {"macro_323"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`timeout`](super::ActorFutureExt::timeout) combinator, interrupts computations if it takes"] # [doc = " more than [`timeout`](super::ActorFutureExt::timeout)."] # [doc = ""] # [doc = " This is created by the [`timeout`](super::ActorFutureExt::timeout) method."] # [derive (Debug)] # [must_use = "futures do nothing unless polled"] pub struct Timeout < F > { # [pin] fut : F , # [pin] timeout : Sleep , } }
};
}
