// Generated macro for render_exec_call (function)
macro_rules! Depcrate_renderrender_exec_call {
() => {
// Module: crate::render
// Provides: {"render_exec_call"}
// Dependencies: {}
fn render_exec_call (fn_path : Path , args : & [Expr] , is_async : bool) -> TokenStream { if is_async { quote ! { # fn_path (# (# args) ,*) . await } } else { quote ! { # fn_path (# (# args) ,*) } } }
};
}
