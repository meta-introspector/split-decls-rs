// Generated macro for TokenPayload (enum)
macro_rules! Depcrate_tokenTokenPayload {
() => {
// Module: crate::token
// Provides: {"TokenPayload"}
// Dependencies: {}
# [doc = " Content of a [`Token`] that is encrypted from the client"] pub (crate) enum TokenPayload { # [doc = " Token originating from a Retry packet"] Retry { # [doc = " The client's address"] address : SocketAddr , # [doc = " The destination connection ID set in the very first packet from the client"] orig_dst_cid : ConnectionId , # [doc = " The time at which this token was issued"] issued : SystemTime , } , # [doc = " Token originating from a NEW_TOKEN frame"] Validation { # [doc = " The client's IP address (its port is likely to change between sessions)"] ip : IpAddr , # [doc = " The time at which this token was issued"] issued : SystemTime , } , }
};
}
