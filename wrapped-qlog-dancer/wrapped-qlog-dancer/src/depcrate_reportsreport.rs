// Generated macro for report (function)
macro_rules! Depcrate_reportsreport {
() => {
// Module: crate::reports
// Provides: {"report"}
// Dependencies: {}
pub fn report (log_file : & LogFileParseResult , config : & AppConfig) { if config . report_text { for data in & log_file . data { if let Some (table) = text :: request_timing_table (data , config) { println ! ("Request timing table for session ID: {:?}, app proto: {:?}, host: {:?}" , & data . datastore . session_id . unwrap_or (- 1) , & data . datastore . application_proto , & data . datastore . host . clone () . unwrap_or ("ERROR UNKNOWN" . to_string ())) ; println ! ("{}" , table) ; println ! () ; } text :: print_stats (& data . datastore , & config . stats_config) ; println ! () ; match & data . raw { crate :: RawLogEvents :: QlogJson { events : _ } => todo ! () , crate :: RawLogEvents :: QlogJsonSeq { events } => { let mut table = sqlog_event_list (events) . build () ; table . with (Style :: sharp ()) ; println ! ("Qlog events") ; println ! ("{}" , table) ; } , crate :: RawLogEvents :: Netlog => todo ! () , } } text :: print_packet_loss (& log_file . data) ; text :: print_flow_control (& log_file . data) ; } if config . report_html { html :: overview (log_file , config) ; html :: closures (log_file , config) ; html :: requests (log_file , config) ; html :: event_list (log_file , config) ; } }
};
}
