// Generated macro for FinalizedBuffers (struct)
macro_rules! Depcrate_export_blob_exporterFinalizedBuffers {
() => {
// Module: crate::export::blob_exporter
// Provides: {"FinalizedBuffers"}
// Dependencies: {}
struct FinalizedBuffers { # [doc = " Sorted list of blob to old ID; the index in the vec is the new ID"] vzv : VarZeroVec < 'static , [u8] , Index32 > , # [doc = " Map from old ID to new ID"] remap : HashMap < usize , usize > , }
};
}
