// Generated macro for ok (function)
macro_rules! Depcrate_fut_future_resultok {
() => {
// Module: crate::fut::future::result
// Provides: {"ok"}
// Dependencies: {}
# [doc = " Creates a \"leaf future\" from an immediate value of a finished and"] # [doc = " successful computation."] # [doc = ""] # [doc = " The returned future is similar to `result` where it will immediately run a"] # [doc = " scheduled callback with the provided value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use actix::fut::*;"] # [doc = " use actix::{Actor, Context};"] # [doc = ""] # [doc = " struct MyActor;"] # [doc = " impl Actor for MyActor {"] # [doc = "     type Context = Context<Self>;"] # [doc = " }"] # [doc = ""] # [doc = " let future_of_1 = ok::<u32, u32>(1);"] # [doc = " ```"] pub fn ok < T , E > (t : T) -> Ready < Result < T , E > > { ready (Ok (t)) }
};
}
