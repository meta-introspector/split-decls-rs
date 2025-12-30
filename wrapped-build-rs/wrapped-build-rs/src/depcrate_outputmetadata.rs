// Generated macro for metadata (function)
macro_rules! Depcrate_outputmetadata {
() => {
// Module: crate::output
// Provides: {"metadata"}
// Dependencies: {}
# [doc = " Metadata, used by `links` scripts."] # [track_caller] pub fn metadata (key : & str , val : & str) { if ! is_ascii_ident (key) { panic ! ("cannot emit metadata: invalid key {key:?}") ; } if val . contains ('\n') { panic ! ("cannot emit metadata: invalid value {val:?}") ; } emit ("metadata" , format_args ! ("{key}={val}")) ; }
};
}
