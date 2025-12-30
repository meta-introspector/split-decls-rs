// Generated macro for CidState (struct)
macro_rules! Depcrate_connection_cid_stateCidState {
() => {
// Module: crate::connection::cid_state
// Provides: {"CidState"}
// Dependencies: {}
# [doc = " Local connection ID management"] pub (super) struct CidState { # [doc = " Timestamp when issued cids should be retired"] retire_timestamp : VecDeque < CidTimestamp > , # [doc = " Number of local connection IDs that have been issued in NEW_CONNECTION_ID frames."] issued : u64 , # [doc = " Sequence numbers of local connection IDs not yet retired by the peer"] active_seq : FxHashSet < u64 > , # [doc = " Sequence number the peer has already retired all CIDs below at our request via `retire_prior_to`"] prev_retire_seq : u64 , # [doc = " Sequence number to set in retire_prior_to field in NEW_CONNECTION_ID frame"] retire_seq : u64 , # [doc = " cid length used to decode short packet"] cid_len : usize , cid_lifetime : Option < Duration > , }
};
}
