// Generated macro for ResponseFuture (type)
macro_rules! Depcrate_handlerResponseFuture {
() => {
// Module: crate::handler
// Provides: {"ResponseFuture"}
// Dependencies: {}
# [doc = " A specialized future for asynchronous message handling."] # [doc = ""] # [doc = " Intended be used for when the future returned doesn't"] # [doc = " need to access Actor's internal state or context to progress, either"] # [doc = " because it's completely agnostic to it, or because the required data has"] # [doc = " already been moved inside the future and it won't need Actor state to continue."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```no_run"] # [doc = " use actix::prelude::*;"] # [doc = ""] # [doc = " #[derive(Message)]"] # [doc = " #[rtype(result = \"Result<(), ()>\")]"] # [doc = " struct Msg;"] # [doc = ""] # [doc = " struct MyActor;"] # [doc = ""] # [doc = " impl Actor for MyActor {"] # [doc = "     type Context = Context<Self>;"] # [doc = " }"] # [doc = ""] # [doc = " impl Handler<Msg> for MyActor {"] # [doc = "     type Result = ResponseFuture<Result<(), ()>>;"] # [doc = ""] # [doc = "     fn handle(&mut self, _: Msg, _: &mut Context<Self>) -> Self::Result {"] # [doc = "         Box::pin(async move {"] # [doc = "             // Some async computation"] # [doc = "             Ok(())"] # [doc = "         })"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] pub type ResponseFuture < I > = Pin < Box < dyn Future < Output = I > > > ;
};
}
