// Generated macro for rustc_link_search_kind (function)
macro_rules! Depcrate_outputrustc_link_search_kind {
() => {
// Module: crate::output
// Provides: {"rustc_link_search_kind"}
// Dependencies: {}
# [doc = " Like [`rustc_link_search`], but with KIND specified separately."] # [track_caller] pub fn rustc_link_search_kind (kind : & str , path : impl AsRef < Path >) { if kind . contains (['=' , '\n']) { panic ! ("cannot emit rustc-link-search: invalid kind {kind:?}") ; } let Some (path) = path . as_ref () . to_str () else { panic ! ("cannot emit rustc-link-search: path is not UTF-8") ; } ; if path . contains ('\n') { panic ! ("cannot emit rustc-link-search: path contains newline") ; } emit ("rustc-link-search" , format_args ! ("{kind}={path}")) ; }
};
}
