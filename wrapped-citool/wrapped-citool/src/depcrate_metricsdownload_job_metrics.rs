// Generated macro for download_job_metrics (function)
macro_rules! Depcrate_metricsdownload_job_metrics {
() => {
// Module: crate::metrics
// Provides: {"download_job_metrics"}
// Dependencies: {}
pub fn download_job_metrics (job_name : & str , sha : & str) -> anyhow :: Result < JsonRoot > { let cache_path = PathBuf :: from (".citool-cache") . join (sha) . join (format ! ("{job_name}.json")) ; if cache_path . is_file () { if let Ok (metrics) = std :: fs :: read_to_string (& cache_path) . map_err (| err | err . into ()) . and_then (| data | anyhow :: Ok :: < JsonRoot > (serde_json :: from_str :: < JsonRoot > (& data) ?)) { return Ok (metrics) ; } } let url = get_metrics_url (job_name , sha) ; let mut response = ureq :: get (& url) . call () ? ; if ! response . status () . is_success () { return Err (anyhow :: anyhow ! ("Cannot fetch metrics from {url}: {}\n{}" , response . status () , response . body_mut () . read_to_string () ?)) ; } let data : JsonRoot = response . body_mut () . read_json () . with_context (| | anyhow :: anyhow ! ("cannot deserialize metrics from {url}")) ? ; if let Ok (_) = std :: fs :: create_dir_all (cache_path . parent () . unwrap ()) { if let Ok (data) = serde_json :: to_string (& data) { let _ = std :: fs :: write (cache_path , data) ; } } Ok (data) }
};
}
