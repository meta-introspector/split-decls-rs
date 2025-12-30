// Generated macro for get_async_fn_body (function)
macro_rules! Depcrateget_async_fn_body {
() => {
// Module: crate
// Provides: {"get_async_fn_body"}
// Dependencies: {}
# [doc = " Peels away all the compiler generated code surrounding the body of an async function,"] pub fn get_async_fn_body < 'tcx > (tcx : TyCtxt < 'tcx > , body : & Body < '_ >) -> Option < & 'tcx Expr < 'tcx > > { get_async_closure_expr (tcx , body . value) }
};
}
