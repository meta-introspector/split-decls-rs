// Generated macro for Handler (struct)
macro_rules! DepcrateHandler {
() => {
// Module: crate
// Provides: {"Handler"}
// Dependencies: {}
# [doc = " A custom handler type for [`eyre::Report`] which provides colorful error"] # [doc = " reports and [`tracing-error`] support."] # [doc = ""] # [doc = " # Details"] # [doc = ""] # [doc = " This type is not intended to be used directly, prefer using it via the"] # [doc = " [`color_eyre::Report`] and [`color_eyre::Result`] type aliases."] # [doc = ""] # [doc = " [`eyre::Report`]: https://docs.rs/eyre/*/eyre/struct.Report.html"] # [doc = " [`tracing-error`]: https://docs.rs/tracing-error"] # [doc = " [`color_eyre::Report`]: type.Report.html"] # [doc = " [`color_eyre::Result`]: type.Result.html"] pub struct Handler { filters : Arc < [Box < config :: FilterCallback >] > , backtrace : Option < Backtrace > , suppress_backtrace : bool , # [cfg (feature = "capture-spantrace")] span_trace : Option < SpanTrace > , sections : Vec < HelpInfo > , display_env_section : bool , # [cfg (feature = "track-caller")] display_location_section : bool , # [cfg (feature = "issue-url")] issue_url : Option < String > , # [cfg (feature = "issue-url")] issue_metadata : std :: sync :: Arc < Vec < (String , Box < dyn std :: fmt :: Display + Send + Sync + 'static >) > > , # [cfg (feature = "issue-url")] issue_filter : std :: sync :: Arc < config :: IssueFilterCallback > , theme : crate :: config :: Theme , # [cfg (feature = "track-caller")] location : Option < & 'static std :: panic :: Location < 'static > > , }
};
}
