// Generated macro for Connection (struct)
macro_rules! DepcrateConnection {
() => {
// Module: crate
// Provides: {"Connection"}
// Dependencies: {}
# [doc = " Connection is just a pair of channels of LSP messages."] pub struct Connection { pub sender : Sender < Message > , pub receiver : Receiver < Message > , }
};
}
