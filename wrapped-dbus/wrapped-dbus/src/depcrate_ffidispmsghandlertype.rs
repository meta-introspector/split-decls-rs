// Generated macro for MsgHandlerType (enum)
macro_rules! Depcrate_ffidispMsgHandlerType {
() => {
// Module: crate::ffidisp
// Provides: {"MsgHandlerType"}
// Dependencies: {}
# [derive (Clone , Debug)] # [doc = " Type of messages to be handled by a MsgHandler."] # [doc = ""] # [doc = " Note: More variants can be added in the future; but unless you're writing your own D-Bus engine"] # [doc = " you should not have to match on these anyway."] pub enum MsgHandlerType { # [doc = " Handle all messages"] All , # [doc = " Handle only messages of a specific type"] MsgType (MessageType) , # [doc = " Handle only method replies with this serial number"] Reply (u32) , }
};
}
