// Generated macro for Blob (struct)
macro_rules! Depcrate_blobBlob {
() => {
// Module: crate::blob
// Provides: {"Blob"}
// Dependencies: {}
# [doc = " Handle to an open BLOB. See"] # [doc = " [`rusqlite::blob`](crate::blob) documentation for in-depth discussion."] pub struct Blob < 'conn > { conn : & 'conn Connection , blob : * mut ffi :: sqlite3_blob , pos : i32 , }
};
}
