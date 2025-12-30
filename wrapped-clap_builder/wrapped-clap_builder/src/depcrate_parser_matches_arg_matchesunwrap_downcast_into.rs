// Generated macro for unwrap_downcast_into (function)
macro_rules! Depcrate_parser_matches_arg_matchesunwrap_downcast_into {
() => {
// Module: crate::parser::matches::arg_matches
// Provides: {"unwrap_downcast_into"}
// Dependencies: {}
# [track_caller] fn unwrap_downcast_into < T : Any + Clone + Send + Sync + 'static > (value : AnyValue) -> T { value . downcast_into () . expect (INTERNAL_ERROR_MSG) }
};
}
