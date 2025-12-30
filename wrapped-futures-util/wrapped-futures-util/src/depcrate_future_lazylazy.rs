// Generated macro for lazy (function)
macro_rules! Depcrate_future_lazylazy {
() => {
// Module: crate::future::lazy
// Provides: {"lazy"}
// Dependencies: {}
# [doc = " Creates a new future that allows delayed execution of a closure."] # [doc = ""] # [doc = " The provided closure is only run once the future is polled."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use futures::future;"] # [doc = ""] # [doc = " let a = future::lazy(|_| 1);"] # [doc = " assert_eq!(a.await, 1);"] # [doc = ""] # [doc = " let b = future::lazy(|_| -> i32 {"] # [doc = "     panic!(\"oh no!\")"] # [doc = " });"] # [doc = " drop(b); // closure is never run"] # [doc = " # });"] # [doc = " ```"] pub fn lazy < F , R > (f : F) -> Lazy < F > where F : FnOnce (& mut Context < '_ >) -> R , { assert_future :: < R , _ > (Lazy { f : Some (f) }) }
};
}
