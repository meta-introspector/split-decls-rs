// Generated macro for is_binary_buf (function)
macro_rules! Depcrate_blob_pipelineis_binary_buf {
() => {
// Module: crate::blob::pipeline
// Provides: {"is_binary_buf"}
// Dependencies: {}
fn is_binary_buf (buf : & [u8]) -> bool { let buf = & buf [.. buf . len () . min (8000)] ; buf . contains (& 0) }
};
}
