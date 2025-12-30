// Generated macro for err (function)
macro_rules! Depcrate_fut_future_resulterr {
() => {
// Module: crate::fut::future::result
// Provides: {"err"}
// Dependencies: {}
# [doc = " Creates a \"leaf future\" from an immediate value of a failed computation."] # [doc = ""] # [doc = " The returned future is similar to `result` where it will immediately run a"] # [doc = " scheduled callback with the provided value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use actix::{fut, Actor, Context};"] # [doc = ""] # [doc = " struct MyActor;"] # [doc = " impl Actor for MyActor {"] # [doc = "     type Context = Context<Self>;"] # [doc = " }"] # [doc = ""] # [doc = " let future_of_err_1 = fut::err::<u32, u32>(1);"] # [doc = " ```"] pub fn err < T , E > (e : E) -> Ready < Result < T , E > > { ready (Err (e)) }
};
}
