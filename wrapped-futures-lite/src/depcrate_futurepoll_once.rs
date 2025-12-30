// Generated macro for poll_once (function)
macro_rules! Depcrate_futurepoll_once {
() => {
// Module: crate::future
// Provides: {"poll_once"}
// Dependencies: {}
# [doc = " Polls a future just once and returns an [`Option`] with the result."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::future;"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " assert_eq!(future::poll_once(future::pending::<()>()).await, None);"] # [doc = " assert_eq!(future::poll_once(future::ready(42)).await, Some(42));"] # [doc = " # })"] # [doc = " ```"] pub fn poll_once < T , F > (f : F) -> PollOnce < F > where F : Future < Output = T > , { PollOnce { f } }
};
}
