// Generated macro for MsgHandlerResult (struct)
macro_rules! Depcrate_ffidispMsgHandlerResult {
() => {
// Module: crate::ffidisp
// Provides: {"MsgHandlerResult"}
// Dependencies: {}
# [doc = " The result from MsgHandler::handle."] # [derive (Debug , Default)] pub struct MsgHandlerResult { # [doc = " Indicates that the message has been dealt with and should not be processed further."] pub handled : bool , # [doc = " Indicates that this MsgHandler no longer wants to receive messages and should be removed."] pub done : bool , # [doc = " Messages to send (e g, a reply to a method call)"] pub reply : Vec < Message > , }
};
}
