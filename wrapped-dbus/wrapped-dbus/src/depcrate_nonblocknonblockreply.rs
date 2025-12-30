// Generated macro for NonblockReply (trait)
macro_rules! Depcrate_nonblockNonblockReply {
() => {
// Module: crate::nonblock
// Provides: {"NonblockReply"}
// Dependencies: {}
# [doc = " Internal helper trait for async method replies."] pub trait NonblockReply { # [doc = " Callback type"] type F ; # [doc = " Sends a message and calls the callback when a reply is received."] fn send_with_reply (& self , msg : Message , f : Self :: F) -> Result < Token , () > ; # [doc = " Cancels a pending reply."] fn cancel_reply (& self , id : Token) -> Option < Self :: F > ; # [doc = " Internal helper function that creates a callback."] fn make_f < G : FnOnce (Message , & Self) + Send + 'static > (g : G) -> Self :: F where Self : Sized ; # [doc = " Set the internal timeout maker"] fn set_timeout_maker (& mut self , f : Option < TimeoutMakerCb >) -> Option < TimeoutMakerCb > ; # [doc = " Get the internal timeout maker"] fn timeout_maker (& self) -> Option < TimeoutMakerCb > ; # [doc = " Set the wakeup call"] fn set_waker (& mut self , f : Option < WakerCb >) -> Option < WakerCb > ; }
};
}
