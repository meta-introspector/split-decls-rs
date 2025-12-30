// Generated macro for ConnectionMeta (struct)
macro_rules! Depcrate_endpointConnectionMeta {
() => {
// Module: crate::endpoint
// Provides: {"ConnectionMeta"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct ConnectionMeta { init_cid : ConnectionId , # [doc = " Number of local connection IDs that have been issued in NEW_CONNECTION_ID frames."] cids_issued : u64 , loc_cids : FxHashMap < u64 , ConnectionId > , # [doc = " Remote/local addresses the connection began with"] # [doc = ""] # [doc = " Only needed to support connections with zero-length CIDs, which cannot migrate, so we don't"] # [doc = " bother keeping it up to date."] addresses : FourTuple , side : Side , # [doc = " Reset token provided by the peer for the CID we're currently sending to, and the address"] # [doc = " being sent to"] reset_token : Option < (SocketAddr , ResetToken) > , }
};
}
