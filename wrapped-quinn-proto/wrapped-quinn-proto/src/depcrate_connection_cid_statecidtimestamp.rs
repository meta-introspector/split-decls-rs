// Generated macro for CidTimestamp (struct)
macro_rules! Depcrate_connection_cid_stateCidTimestamp {
() => {
// Module: crate::connection::cid_state
// Provides: {"CidTimestamp"}
// Dependencies: {}
# [doc = " Data structure that records when issued cids should be retired"] # [derive (Copy , Clone , Eq , PartialEq)] struct CidTimestamp { # [doc = " Highest cid sequence number created in a batch"] sequence : u64 , # [doc = " Timestamp when cid needs to be retired"] timestamp : Instant , }
};
}
