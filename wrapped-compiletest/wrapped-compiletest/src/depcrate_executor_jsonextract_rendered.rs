// Generated macro for extract_rendered (function)
macro_rules! Depcrate_executor_jsonextract_rendered {
() => {
// Module: crate::executor::json
// Provides: {"extract_rendered"}
// Dependencies: {}
pub fn extract_rendered (output : & str) -> String { output . lines () . filter_map (| line | { if line . starts_with ('{') { if let Ok (diagnostic) = serde_json :: from_str :: < Diagnostic > (line) { diagnostic . rendered } else if let Ok (report) = serde_json :: from_str :: < FutureIncompatReport > (line) { if report . future_incompat_report . is_empty () { None } else { Some (format ! ("Future incompatibility report: {}" , report . future_incompat_report . into_iter () . map (| item | { format ! ("Future breakage diagnostic:\n{}" , item . diagnostic . rendered . unwrap_or_else (|| "Not rendered" . to_string ())) }) . collect ::< String > ())) } } else if serde_json :: from_str :: < ArtifactNotification > (line) . is_ok () { None } else if serde_json :: from_str :: < UnusedExternNotification > (line) . is_ok () { None } else { Some (format ! ("{line}\n")) } } else { Some (format ! ("{}\n" , line)) } }) . collect () }
};
}
