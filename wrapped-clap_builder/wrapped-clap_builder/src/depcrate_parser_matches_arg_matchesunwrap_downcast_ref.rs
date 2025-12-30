// Generated macro for unwrap_downcast_ref (function)
macro_rules! Depcrate_parser_matches_arg_matchesunwrap_downcast_ref {
() => {
// Module: crate::parser::matches::arg_matches
// Provides: {"unwrap_downcast_ref"}
// Dependencies: {}
# [track_caller] fn unwrap_downcast_ref < T : Any + Clone + Send + Sync + 'static > (value : & AnyValue) -> & T { value . downcast_ref () . expect (INTERNAL_ERROR_MSG) }
};
}
