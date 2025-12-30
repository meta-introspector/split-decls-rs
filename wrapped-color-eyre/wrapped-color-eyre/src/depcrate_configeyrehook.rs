// Generated macro for EyreHook (struct)
macro_rules! Depcrate_configEyreHook {
() => {
// Module: crate::config
// Provides: {"EyreHook"}
// Dependencies: {}
# [doc = " An eyre reporting hook used to construct `EyreHandler`s"] pub struct EyreHook { filters : Arc < [Box < FilterCallback >] > , # [cfg (feature = "capture-spantrace")] capture_span_trace_by_default : bool , display_env_section : bool , # [cfg (feature = "track-caller")] display_location_section : bool , theme : Theme , # [cfg (feature = "issue-url")] issue_url : Option < String > , # [cfg (feature = "issue-url")] issue_metadata : Arc < Vec < (String , Box < dyn Display + Send + Sync + 'static >) > > , # [cfg (feature = "issue-url")] issue_filter : Arc < IssueFilterCallback > , }
};
}
