// Generated macro for race (function)
macro_rules! Depcrate_futurerace {
() => {
// Module: crate::future
// Provides: {"race"}
// Dependencies: {}
# [doc = " Returns the result of the future that completes first, with no preference if both are ready."] # [doc = ""] # [doc = " Each time [`Race`] is polled, the two inner futures are polled in random order. Therefore, no"] # [doc = " future takes precedence over the other if both can complete at the same time."] # [doc = ""] # [doc = " If you have preference for one of the futures, use the [`or()`] function or the"] # [doc = " [`FutureExt::or()`] method."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::future::{self, pending, ready};"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " assert_eq!(future::race(ready(1), pending()).await, 1);"] # [doc = " assert_eq!(future::race(pending(), ready(2)).await, 2);"] # [doc = ""] # [doc = " // One of the two futures is randomly chosen as the winner."] # [doc = " let res = future::race(ready(1), ready(2)).await;"] # [doc = " # })"] # [doc = " ```"] # [cfg (all (feature = "race" , feature = "std"))] pub fn race < T , F1 , F2 > (future1 : F1 , future2 : F2) -> Race < F1 , F2 > where F1 : Future < Output = T > , F2 : Future < Output = T > , { Race { future1 , future2 , rng : Rng :: new () , } }
};
}
