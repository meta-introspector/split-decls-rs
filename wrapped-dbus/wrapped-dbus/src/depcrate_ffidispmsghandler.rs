// Generated macro for MsgHandler (trait)
macro_rules! Depcrate_ffidispMsgHandler {
() => {
// Module: crate::ffidisp
// Provides: {"MsgHandler"}
// Dependencies: {}
# [doc = " A trait for handling incoming messages."] pub trait MsgHandler { # [doc = " Type of messages for which the handler will be called"] # [doc = ""] # [doc = " Note: The return value of this function might be cached, so it must return the same value all the time."] fn handler_type (& self) -> MsgHandlerType ; # [doc = " Function to be called if the message matches the MsgHandlerType"] fn handle_msg (& mut self , _msg : & Message) -> Option < MsgHandlerResult > { None } }
};
}
