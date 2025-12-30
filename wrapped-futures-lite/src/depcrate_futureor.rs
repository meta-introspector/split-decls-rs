// Generated macro for or (function)
macro_rules! Depcrate_futureor {
() => {
// Module: crate::future
// Provides: {"or"}
// Dependencies: {}
# [doc = " Returns the result of the future that completes first, preferring `future1` if both are ready."] # [doc = ""] # [doc = " If you need to treat the two futures fairly without a preference for either, use the [`race()`]"] # [doc = " function or the [`FutureExt::race()`] method."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::future::{self, pending, ready};"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " assert_eq!(future::or(ready(1), pending()).await, 1);"] # [doc = " assert_eq!(future::or(pending(), ready(2)).await, 2);"] # [doc = ""] # [doc = " // The first future wins."] # [doc = " assert_eq!(future::or(ready(1), ready(2)).await, 1);"] # [doc = " # })"] # [doc = " ```"] pub fn or < T , F1 , F2 > (future1 : F1 , future2 : F2) -> Or < F1 , F2 > where F1 : Future < Output = T > , F2 : Future < Output = T > , { Or { future1 , future2 } }
};
}
