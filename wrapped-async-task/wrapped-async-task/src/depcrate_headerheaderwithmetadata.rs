// Generated macro for HeaderWithMetadata (struct)
macro_rules! Depcrate_headerHeaderWithMetadata {
() => {
// Module: crate::header
// Provides: {"HeaderWithMetadata"}
// Dependencies: {}
# [doc = " The header of a task."] # [doc = ""] # [doc = " This header is stored in memory at the beginning of the heap-allocated task."] # [repr (C)] pub (crate) struct HeaderWithMetadata < M > { pub (crate) header : Header , # [doc = " Metadata associated with the task."] # [doc = ""] # [doc = " This metadata may be provided to the user."] pub (crate) metadata : M , }
};
}
