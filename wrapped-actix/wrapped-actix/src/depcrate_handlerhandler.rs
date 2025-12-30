// Generated macro for Handler (trait)
macro_rules! Depcrate_handlerHandler {
() => {
// Module: crate::handler
// Provides: {"Handler"}
// Dependencies: {}
# [doc = " Describes how to handle messages of a specific type."] # [doc = ""] # [doc = " Implementing `Handler` is a general way to handle incoming"] # [doc = " messages, streams, and futures."] # [doc = ""] # [doc = " The type `M` is a message which can be handled by the actor."] # [allow (unused_variables)] pub trait Handler < M > where Self : Actor , M : Message , { # [doc = " The type of value that this handler will return."] # [doc = ""] # [doc = " Check the [`MessageResponse`] trait for some details"] # [doc = " on how a message can be responded to."] type Result : MessageResponse < Self , M > ; # [doc = " This method is called for every message received by this actor."] fn handle (& mut self , msg : M , ctx : & mut Self :: Context) -> Self :: Result ; }
};
}
