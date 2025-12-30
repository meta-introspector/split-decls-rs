// Generated macro for set_call_limit (function)
macro_rules! Depcrate_parser_stateset_call_limit {
() => {
// Module: crate::parser_state
// Provides: {"set_call_limit"}
// Dependencies: {}
# [doc = " Sets the maximum call limit for the parser state"] # [doc = " to prevent stack overflows or excessive execution times"] # [doc = " in some grammars."] # [doc = " If set, the calls are tracked as a running total"] # [doc = " over all non-terminal rules that can nest closures"] # [doc = " (which are passed to transform the parser state)."] # [doc = ""] # [doc = " # Arguments"] # [doc = ""] # [doc = " * `limit` - The maximum number of calls. If None,"] # [doc = "             the number of calls is unlimited."] pub fn set_call_limit (limit : Option < NonZeroUsize >) { CALL_LIMIT . store (limit . map (| f | f . get ()) . unwrap_or (0) , Ordering :: Relaxed) ; }
};
}
