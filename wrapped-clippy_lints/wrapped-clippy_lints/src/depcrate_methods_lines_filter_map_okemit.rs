// Generated macro for emit (function)
macro_rules! Depcrate_methods_lines_filter_map_okemit {
() => {
// Module: crate::methods::lines_filter_map_ok
// Provides: {"emit"}
// Dependencies: {}
fn emit (cx : & LateContext < '_ > , recv : & Expr < '_ > , method_name : & 'static str , call_span : Span) { span_lint_and_then (cx , LINES_FILTER_MAP_OK , call_span , format ! ("`{method_name}()` will run forever if the iterator repeatedly produces an `Err`") , | diag | { diag . span_note (recv . span , "this expression returning a `std::io::Lines` may produce \
                        an infinite number of `Err` in case of a read error" ,) ; diag . span_suggestion (call_span , "replace with" , "map_while(Result::ok)" , Applicability :: MaybeIncorrect ,) ; } ,) ; }
};
}
