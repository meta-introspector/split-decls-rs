// Generated macro for IssueSection (struct)
macro_rules! Depcrate_section_githubIssueSection {
() => {
// Module: crate::section::github
// Provides: {"IssueSection"}
// Dependencies: {}
pub (crate) struct IssueSection < 'a > { url : & 'a str , msg : & 'a str , location : Option < & 'a Location < 'a > > , backtrace : Option < & 'a Backtrace > , # [cfg (feature = "capture-spantrace")] span_trace : Option < & 'a SpanTrace > , metadata : & 'a [(String , Display < 'a >)] , }
};
}
