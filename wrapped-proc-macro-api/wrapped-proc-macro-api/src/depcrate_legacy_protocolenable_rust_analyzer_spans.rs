// Generated macro for enable_rust_analyzer_spans (function)
macro_rules! Depcrate_legacy_protocolenable_rust_analyzer_spans {
() => {
// Module: crate::legacy_protocol
// Provides: {"enable_rust_analyzer_spans"}
// Dependencies: {}
# [doc = " Enable support for rust-analyzer span mode if the server supports it."] pub (crate) fn enable_rust_analyzer_spans (srv : & ProcMacroServerProcess ,) -> Result < SpanMode , ServerError > { let request = Request :: SetConfig (ServerConfig { span_mode : SpanMode :: RustAnalyzer }) ; let response = send_task (srv , request) ? ; match response { Response :: SetConfig (ServerConfig { span_mode }) => Ok (span_mode) , _ => Err (ServerError { message : "unexpected response" . to_owned () , io : None }) , } }
};
}
