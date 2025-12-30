// Generated macro for MetadataHandle (struct)
macro_rules! Depcrate_shims_windows_fsMetadataHandle {
() => {
// Module: crate::shims::windows::fs
// Provides: {"MetadataHandle"}
// Dependencies: {}
# [doc = " Windows supports handles without any read/write/delete permissions - these handles can get"] # [doc = " metadata, but little else. We represent that by storing the metadata from the time the handle"] # [doc = " was opened."] # [derive (Debug)] pub struct MetadataHandle { pub (crate) meta : Metadata , }
};
}
