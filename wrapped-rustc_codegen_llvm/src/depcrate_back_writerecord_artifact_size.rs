// Generated macro for record_artifact_size (function)
macro_rules! Depcrate_back_writerecord_artifact_size {
() => {
// Module: crate::back::write
// Provides: {"record_artifact_size"}
// Dependencies: {}
fn record_artifact_size (self_profiler_ref : & SelfProfilerRef , artifact_kind : & 'static str , path : & Path ,) { if ! self_profiler_ref . enabled () { return ; } if let Some (artifact_name) = path . file_name () { let file_size = std :: fs :: metadata (path) . map (| m | m . len ()) . unwrap_or (0) ; self_profiler_ref . artifact_size (artifact_kind , artifact_name . to_string_lossy () , file_size) ; } }
};
}
