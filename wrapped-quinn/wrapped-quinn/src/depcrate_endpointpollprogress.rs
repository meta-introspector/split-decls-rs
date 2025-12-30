// Generated macro for PollProgress (struct)
macro_rules! Depcrate_endpointPollProgress {
() => {
// Module: crate::endpoint
// Provides: {"PollProgress"}
// Dependencies: {}
# [derive (Default)] struct PollProgress { # [doc = " Whether a datagram was routed to an existing connection"] received_connection_packet : bool , # [doc = " Whether datagram handling was interrupted early by the work limiter for fairness"] keep_going : bool , }
};
}
