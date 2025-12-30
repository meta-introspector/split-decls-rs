// Generated macro for DBusMessageType (enum)
macro_rules! DepcrateDBusMessageType {
() => {
// Module: crate
// Provides: {"DBusMessageType"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , PartialEq , Copy , Clone)] # [doc = " One of the four different D-Bus message types."] pub enum DBusMessageType { # [doc = " This is not a valid message type (rarely used)"] Invalid = 0 , # [doc = " This is a method call D-Bus message"] MethodCall = 1 , # [doc = " This is a method return Ok D-Bus message, used when the method call message was successfully processed"] MethodReturn = 2 , # [doc = " This is a method return with error D-Bus message, used when the method call message could not be handled "] Error = 3 , # [doc = " This is a signal, usually sent to whoever wants to listen"] Signal = 4 , }
};
}
