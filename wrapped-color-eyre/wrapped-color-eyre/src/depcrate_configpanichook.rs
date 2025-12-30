// Generated macro for PanicHook (struct)
macro_rules! Depcrate_configPanicHook {
() => {
// Module: crate::config
// Provides: {"PanicHook"}
// Dependencies: {}
# [doc = " A panic reporting hook"] pub struct PanicHook { filters : Arc < [Box < FilterCallback >] > , section : Option < Box < dyn Display + Send + Sync + 'static > > , panic_message : Box < dyn PanicMessage > , theme : Theme , # [cfg (feature = "capture-spantrace")] capture_span_trace_by_default : bool , display_env_section : bool , # [cfg (feature = "issue-url")] issue_url : Option < String > , # [cfg (feature = "issue-url")] issue_metadata : Arc < Vec < (String , Box < dyn Display + Send + Sync + 'static >) > > , # [cfg (feature = "issue-url")] issue_filter : Arc < IssueFilterCallback > , }
};
}
