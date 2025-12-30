// Generated macro for maybe_done (function)
macro_rules! Depcrate_future_maybe_donemaybe_done {
() => {
// Module: crate::future::maybe_done
// Provides: {"maybe_done"}
// Dependencies: {}
# [doc = " Wraps a future into a `MaybeDone`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use core::pin::pin;"] # [doc = ""] # [doc = " use futures::future;"] # [doc = ""] # [doc = " let future = future::maybe_done(async { 5 });"] # [doc = " let mut future = pin!(future);"] # [doc = " assert_eq!(future.as_mut().take_output(), None);"] # [doc = " let () = future.as_mut().await;"] # [doc = " assert_eq!(future.as_mut().take_output(), Some(5));"] # [doc = " assert_eq!(future.as_mut().take_output(), None);"] # [doc = " # });"] # [doc = " ```"] pub fn maybe_done < Fut : Future > (future : Fut) -> MaybeDone < Fut > { assert_future :: < () , _ > (MaybeDone :: Future (future)) }
};
}
