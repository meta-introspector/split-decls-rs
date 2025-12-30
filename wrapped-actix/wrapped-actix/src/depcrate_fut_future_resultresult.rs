// Generated macro for result (function)
macro_rules! Depcrate_fut_future_resultresult {
() => {
// Module: crate::fut::future::result
// Provides: {"result"}
// Dependencies: {}
# [doc = " Creates a new \"leaf future\" which will resolve with the given result."] # [doc = ""] # [doc = " The returned future represents a computation which is finished immediately."] # [doc = " This can be useful with the `finished` and `failed` base future types to"] # [doc = " convert an immediate value to a future to interoperate elsewhere."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use actix::{fut, Actor, Context};"] # [doc = ""] # [doc = " struct MyActor;"] # [doc = " impl Actor for MyActor {"] # [doc = "     type Context = Context<Self>;"] # [doc = " }"] # [doc = ""] # [doc = " let future_of_1 = fut::result::<u32, u32>(Ok(1));"] # [doc = " let future_of_err_2 = fut::result::<u32, u32>(Err(2));"] # [doc = " ```"] pub fn result < T , E > (r : Result < T , E >) -> Ready < Result < T , E > > { ready (r) }
};
}
