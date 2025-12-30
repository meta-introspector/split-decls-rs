// Generated macro for impl_83 (impl)
macro_rules! Depcrate_section_githubimpl_83 {
() => {
// Module: crate::section::github
// Provides: {"impl_83"}
// Dependencies: {}
impl < 'a > IssueSection < 'a > { pub (crate) fn new (url : & 'a str , msg : & 'a str) -> Self { IssueSection { url , msg , location : None , backtrace : None , # [cfg (feature = "capture-spantrace")] span_trace : None , metadata : & [] , } } pub (crate) fn with_location (mut self , location : impl Into < Option < & 'a Location < 'a > > >) -> Self { self . location = location . into () ; self } pub (crate) fn with_backtrace (mut self , backtrace : impl Into < Option < & 'a Backtrace > >) -> Self { self . backtrace = backtrace . into () ; self } # [cfg (feature = "capture-spantrace")] pub (crate) fn with_span_trace (mut self , span_trace : impl Into < Option < & 'a SpanTrace > >) -> Self { self . span_trace = span_trace . into () ; self } pub (crate) fn with_metadata (mut self , metadata : & 'a [(String , Display < 'a >)]) -> Self { self . metadata = metadata ; self } }
};
}
