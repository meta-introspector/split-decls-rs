// Generated macro for Ready (struct)
macro_rules! Depcrate_future_readyReady {
() => {
// Module: crate::future::ready
// Provides: {"Ready"}
// Dependencies: {}
# [doc = " Future for the [`ready`] function."] # [doc = ""] # [doc = " Panic will occur if polled more than once."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " use actix_utils::future::ready;"] # [doc = ""] # [doc = " // async"] # [doc = " # async fn run() {"] # [doc = " let a = ready(1);"] # [doc = " assert_eq!(a.await, 1);"] # [doc = " # }"] # [doc = ""] # [doc = " // sync"] # [doc = " let a = ready(1);"] # [doc = " assert_eq!(a.into_inner(), 1);"] # [doc = " ```"] # [derive (Debug , Clone)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Ready < T > { val : Option < T > , }
};
}
