// Generated macro for Header (struct)
macro_rules! Depcrate_decodeHeader {
() => {
// Module: crate::decode
// Provides: {"Header"}
// Dependencies: {}
# [doc = " Blobby file header"] pub struct Header { # [doc = " Number of blobs stored in the file"] pub items_len : usize , # [doc = " Number of deduplicated blobs"] pub dedup_len : usize , }
};
}
