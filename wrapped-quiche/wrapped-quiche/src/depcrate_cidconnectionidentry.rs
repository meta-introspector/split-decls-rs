// Generated macro for ConnectionIdEntry (struct)
macro_rules! Depcrate_cidConnectionIdEntry {
() => {
// Module: crate::cid
// Provides: {"ConnectionIdEntry"}
// Dependencies: {}
# [doc = " A structure holding a `ConnectionId` and all its related metadata."] # [derive (Debug , Default)] pub struct ConnectionIdEntry { # [doc = " The Connection ID."] pub cid : ConnectionId < 'static > , # [doc = " Its associated sequence number."] pub seq : u64 , # [doc = " Its associated reset token. Initial CIDs may not have any reset token."] pub reset_token : Option < u128 > , # [doc = " The path identifier using this CID, if any."] pub path_id : Option < usize > , }
};
}
