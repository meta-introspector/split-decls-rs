// Generated macro for CidQueue (struct)
macro_rules! Depcrate_cid_queueCidQueue {
() => {
// Module: crate::cid_queue
// Provides: {"CidQueue"}
// Dependencies: {}
# [doc = " Sliding window of active Connection IDs"] # [doc = ""] # [doc = " May contain gaps due to packet loss or reordering"] # [derive (Debug)] pub (crate) struct CidQueue { # [doc = " Ring buffer indexed by `self.cursor`"] buffer : [Option < CidData > ; Self :: LEN] , # [doc = " Index at which circular buffer addressing is based"] cursor : usize , # [doc = " Sequence number of `self.buffer[cursor]`"] # [doc = ""] # [doc = " The sequence number of the active CID; must be the smallest among CIDs in `buffer`."] offset : u64 , }
};
}
