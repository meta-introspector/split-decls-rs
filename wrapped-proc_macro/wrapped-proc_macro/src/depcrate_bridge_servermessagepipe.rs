// Generated macro for MessagePipe (trait)
macro_rules! Depcrate_bridge_serverMessagePipe {
() => {
// Module: crate::bridge::server
// Provides: {"MessagePipe"}
// Dependencies: {}
# [doc = " A message pipe used for communicating between server and client threads."] pub trait MessagePipe < T > : Sized { # [doc = " Creates a new pair of endpoints for the message pipe."] fn new () -> (Self , Self) ; # [doc = " Send a message to the other endpoint of this pipe."] fn send (& mut self , value : T) ; # [doc = " Receive a message from the other endpoint of this pipe."] # [doc = ""] # [doc = " Returns `None` if the other end of the pipe has been destroyed, and no"] # [doc = " message was received."] fn recv (& mut self) -> Option < T > ; }
};
}
