// Generated macro for ConnectionItem (enum)
macro_rules! Depcrate_ffidispConnectionItem {
() => {
// Module: crate::ffidisp
// Provides: {"ConnectionItem"}
// Dependencies: {}
# [doc = " When listening for incoming events on the D-Bus, this enum will tell you what type"] # [doc = " of incoming event has happened."] # [derive (Debug)] pub enum ConnectionItem { # [doc = " No event between now and timeout"] Nothing , # [doc = " Incoming method call"] MethodCall (Message) , # [doc = " Incoming signal"] Signal (Message) , # [doc = " Incoming method return, including method return errors (mostly used for Async I/O)"] MethodReturn (Message) , }
};
}
