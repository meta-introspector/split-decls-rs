// Generated macro for join (function)
macro_rules! Depcrate_future_joinjoin {
() => {
// Module: crate::future::join
// Provides: {"join"}
// Dependencies: {}
# [doc = " Joins the result of two futures, waiting for them both to complete."] # [doc = ""] # [doc = " This function will return a new future which awaits both futures to"] # [doc = " complete. The returned future will finish with a tuple of both results."] # [doc = ""] # [doc = " Note that this function consumes the passed futures and returns a"] # [doc = " wrapped version of it."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use futures::future;"] # [doc = ""] # [doc = " let a = async { 1 };"] # [doc = " let b = async { 2 };"] # [doc = " let pair = future::join(a, b);"] # [doc = ""] # [doc = " assert_eq!(pair.await, (1, 2));"] # [doc = " # });"] # [doc = " ```"] pub fn join < Fut1 , Fut2 > (future1 : Fut1 , future2 : Fut2) -> Join < Fut1 , Fut2 > where Fut1 : Future , Fut2 : Future , { assert_future :: < (Fut1 :: Output , Fut2 :: Output) , _ > (Join :: new (future1 , future2)) }
};
}
