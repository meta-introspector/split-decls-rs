// Generated macro for Input (enum)
macro_rules! Depcrate_graphql_transport_wsInput {
() => {
// Module: crate::graphql_transport_ws
// Provides: {"Input"}
// Dependencies: {}
# [doc = " Possible inputs received from a client."] # [derive (Debug , From)] pub enum Input < S > { # [doc = " Deserialized [`ClientMessage`]."] Message (ClientMessage < S >) , # [doc = " Client initiated normal closing of a [`Connection`]."] Close , }
};
}
