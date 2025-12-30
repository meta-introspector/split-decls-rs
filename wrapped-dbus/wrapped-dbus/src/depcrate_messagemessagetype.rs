// Generated macro for MessageType (enum)
macro_rules! Depcrate_messageMessageType {
() => {
// Module: crate::message
// Provides: {"MessageType"}
// Dependencies: {}
# [derive (Copy , Clone , Debug , Eq , Ord , PartialEq , PartialOrd , Hash)] # [doc = " One of the four different message types."] pub enum MessageType { # [doc = " This is a method call D-Bus message"] MethodCall = 1 , # [doc = " This is a method return Ok D-Bus message, used when the method call message was successfully processed"] MethodReturn = 2 , # [doc = " This is a method return with error D-Bus message, used when the method call message could not be handled"] Error = 3 , # [doc = " This is a signal, usually sent to whoever wants to listen"] Signal = 4 , }
};
}
