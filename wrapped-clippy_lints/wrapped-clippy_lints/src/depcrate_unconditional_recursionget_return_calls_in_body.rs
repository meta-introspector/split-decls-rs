// Generated macro for get_return_calls_in_body (function)
macro_rules! Depcrate_unconditional_recursionget_return_calls_in_body {
() => {
// Module: crate::unconditional_recursion
// Provides: {"get_return_calls_in_body"}
// Dependencies: {}
fn get_return_calls_in_body < 'tcx > (body : & 'tcx Body < 'tcx >) -> Vec < & 'tcx Expr < 'tcx > > { let mut visitor = ReturnsVisitor :: default () ; visitor . visit_body (body) ; visitor . returns }
};
}
