// Generated macro for ready (function)
macro_rules! Depcrate_future_readyready {
() => {
// Module: crate::future::ready
// Provides: {"ready"}
// Dependencies: {}
# [doc = " Creates a future that is immediately ready with a value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use futures::future;"] # [doc = ""] # [doc = " let a = future::ready(1);"] # [doc = " assert_eq!(a.await, 1);"] # [doc = " # });"] # [doc = " ```"] pub fn ready < T > (t : T) -> Ready < T > { assert_future :: < T , _ > (Ready (Some (t))) }
};
}
