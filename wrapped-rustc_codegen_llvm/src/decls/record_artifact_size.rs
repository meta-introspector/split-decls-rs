macro_rules! record_artifact_size {
    () => {
        fn record_artifact_size (self_profiler_ref : & SelfProfilerRef , artifact_kind : & 'static str , path : & Path ,) { if ! self_profiler_ref . enabled () { return ; } if let Some (artifact_name) = path . file_name () { let file_size = std :: fs :: metadata (path) . map (| m | m . len ()) . unwrap_or (0) ; self_profiler_ref . artifact_size (artifact_kind , artifact_name . to_string_lossy () , file_size) ; } }
    };
}

record_artifact_size!();