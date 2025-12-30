// Generated macro for rustc_link_lib_kind (function)
macro_rules! Depcrate_outputrustc_link_lib_kind {
() => {
// Module: crate::output
// Provides: {"rustc_link_lib_kind"}
// Dependencies: {}
# [doc = " Like [`rustc_link_lib`], but with `KIND[:MODIFIERS]` specified separately."] # [track_caller] pub fn rustc_link_lib_kind (kind : & str , lib : & str) { if kind . contains (['=' , ' ' , '\n']) { panic ! ("cannot emit rustc-link-lib: invalid kind {kind:?}") ; } if lib . contains ([' ' , '\n']) { panic ! ("cannot emit rustc-link-lib: invalid lib {lib:?}") ; } emit ("rustc-link-lib" , format_args ! ("{kind}={lib}")) ; }
};
}
