// Generated macro for always_ready (function)
macro_rules! Depcrate_future_always_readyalways_ready {
() => {
// Module: crate::future::always_ready
// Provides: {"always_ready"}
// Dependencies: {}
# [doc = " Creates a future that is always immediately ready with a value."] # [doc = ""] # [doc = " This is particularly useful in avoiding a heap allocation when an API needs [`Box<dyn Future<Output = T>>`],"] # [doc = " as [`AlwaysReady`] does not have to store a boolean for `is_finished`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use std::mem::size_of_val;"] # [doc = ""] # [doc = " use futures::future;"] # [doc = ""] # [doc = " let a = future::always_ready(|| 1);"] # [doc = " assert_eq!(size_of_val(&a), 0);"] # [doc = " assert_eq!(a.await, 1);"] # [doc = " assert_eq!(a.await, 1);"] # [doc = " # });"] # [doc = " ```"] pub fn always_ready < T , F : Fn () -> T > (prod : F) -> AlwaysReady < T , F > { assert_future :: < T , _ > (AlwaysReady (prod)) }
};
}
