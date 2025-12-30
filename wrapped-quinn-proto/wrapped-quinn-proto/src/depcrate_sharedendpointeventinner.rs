// Generated macro for EndpointEventInner (enum)
macro_rules! Depcrate_sharedEndpointEventInner {
() => {
// Module: crate::shared
// Provides: {"EndpointEventInner"}
// Dependencies: {}
# [derive (Clone , Debug , Eq , PartialEq)] pub (crate) enum EndpointEventInner { # [doc = " The connection has been drained"] Drained , # [doc = " The reset token and/or address eligible for generating resets has been updated"] ResetToken (SocketAddr , ResetToken) , # [doc = " The connection needs connection identifiers"] NeedIdentifiers (Instant , u64) , # [doc = " Stop routing connection ID for this sequence number to the connection"] # [doc = " When `bool == true`, a new connection ID will be issued to peer"] RetireConnectionId (Instant , u64 , bool) , }
};
}
