// Generated macro for Output (enum)
macro_rules! Depcrate_graphql_transport_wsOutput {
() => {
// Module: crate::graphql_transport_ws
// Provides: {"Output"}
// Dependencies: {}
# [doc = " Output provides the responses that should be sent to the client."] # [derive (Debug , PartialEq)] pub enum Output < S : ScalarValue > { # [doc = " Message is a message that should be serialized and sent to the client."] Message (ServerMessage < S >) , # [doc = " Close indicates that the connection should be closed and provides a code and message to"] # [doc = " send to the client. This is always the last message in the output stream."] Close { # [doc = " The WebSocket code that should be sent."] code : u16 , # [doc = " A message describing the reason for the connection closing."] message : String , } , }
};
}
