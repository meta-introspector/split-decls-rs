// Generated macro for create_json_frame (function)
macro_rules! Depcrate_log_json_loggercreate_json_frame {
() => {
// Module: crate::log::json_logger
// Provides: {"create_json_frame"}
// Dependencies: {}
# [doc = " Create a new [JsonFrame] from a log-frame from the target"] fn create_json_frame (record : DefmtRecord , host_timestamp : i64) -> JsonFrame { JsonFrame { data : record . args () . to_string () , host_timestamp , level : record . level () , location : Location { file : record . file () . map (| f | f . to_string ()) , line : record . line () , module_path : create_module_path (record . module_path ()) , } , target_timestamp : record . timestamp () . to_string () , } }
};
}
