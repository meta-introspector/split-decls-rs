// Generated macro for err (function)
macro_rules! Depcrate_future_readyerr {
() => {
// Module: crate::future::ready
// Provides: {"err"}
// Dependencies: {}
# [doc = " Creates a future that is immediately ready with an error value."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```no_run"] # [doc = " use actix_utils::future::err;"] # [doc = ""] # [doc = " # async fn run() {"] # [doc = " let a = err::<(), _>(1);"] # [doc = " assert_eq!(a.await, Err(1));"] # [doc = " # }"] # [doc = " ```"] # [inline] pub fn err < T , E > (err : E) -> Ready < Result < T , E > > { Ready { val : Some (Err (err)) , } }
};
}
