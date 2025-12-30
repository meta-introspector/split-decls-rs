// Generated macro for emit_lint (function)
macro_rules! Depcrate_multiple_bound_locationsemit_lint {
() => {
// Module: crate::multiple_bound_locations
// Provides: {"emit_lint"}
// Dependencies: {}
fn emit_lint (cx : & EarlyContext < '_ > , bound_span : Span , where_span : Span) { span_lint (cx , MULTIPLE_BOUND_LOCATIONS , vec ! [bound_span , where_span] , "bound is defined in more than one place" ,) ; }
};
}
