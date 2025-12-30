// Generated macro for HookBuilder (struct)
macro_rules! Depcrate_configHookBuilder {
() => {
// Module: crate::config
// Provides: {"HookBuilder"}
// Dependencies: {}
# [doc = " Builder for customizing the behavior of the global panic and error report hooks"] pub struct HookBuilder { filters : Vec < Box < FilterCallback > > , capture_span_trace_by_default : bool , display_env_section : bool , # [cfg (feature = "track-caller")] display_location_section : bool , panic_section : Option < Box < dyn Display + Send + Sync + 'static > > , panic_message : Option < Box < dyn PanicMessage > > , theme : Theme , # [cfg (feature = "issue-url")] issue_url : Option < String > , # [cfg (feature = "issue-url")] issue_metadata : Vec < (String , Box < dyn Display + Send + Sync + 'static >) > , # [cfg (feature = "issue-url")] issue_filter : Arc < IssueFilterCallback > , }
};
}
