// Generated macro for ok (function)
macro_rules! Depcrate_future_readyok {
() => {
// Module: crate::future::ready
// Provides: {"ok"}
// Dependencies: {}
# [doc = " Creates a future that is immediately ready with a success value."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```no_run"] # [doc = " use actix_utils::future::ok;"] # [doc = ""] # [doc = " # async fn run() {"] # [doc = " let a = ok::<_, ()>(1);"] # [doc = " assert_eq!(a.await, Ok(1));"] # [doc = " # }"] # [doc = " ```"] # [inline] pub fn ok < T , E > (val : T) -> Ready < Result < T , E > > { Ready { val : Some (Ok (val)) } }
};
}
