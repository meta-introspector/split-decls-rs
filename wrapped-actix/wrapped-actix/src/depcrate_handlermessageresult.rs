// Generated macro for MessageResult (struct)
macro_rules! Depcrate_handlerMessageResult {
() => {
// Module: crate::handler
// Provides: {"MessageResult"}
// Dependencies: {}
# [doc = " A helper type that implements the [`MessageResponse`] trait."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```no_run"] # [doc = " use actix::prelude::*;"] # [doc = ""] # [doc = " #[derive(Message)]"] # [doc = " #[rtype(Response)]"] # [doc = " struct Msg;"] # [doc = ""] # [doc = " struct Response;"] # [doc = ""] # [doc = " struct MyActor;"] # [doc = ""] # [doc = " impl Actor for MyActor {"] # [doc = "     type Context = Context<Self>;"] # [doc = " }"] # [doc = ""] # [doc = " impl Handler<Msg> for MyActor {"] # [doc = "     type Result = MessageResult<Msg>;"] # [doc = ""] # [doc = "     fn handle(&mut self, _: Msg, _: &mut Context<Self>) -> Self::Result {"] # [doc = "         MessageResult(Response {})"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] pub struct MessageResult < M : Message > (pub M :: Result) ;
};
}
