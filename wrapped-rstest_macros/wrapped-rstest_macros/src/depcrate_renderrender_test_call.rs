// Generated macro for render_test_call (function)
macro_rules! Depcrate_renderrender_test_call {
() => {
// Module: crate::render
// Provides: {"render_test_call"}
// Dependencies: {}
fn render_test_call (fn_path : Path , args : & [Expr] , timeout : Option < Expr > , is_async : bool ,) -> TokenStream { let timeout = timeout . map (| x | quote ! { # x }) . or_else (| | { std :: env :: var ("RSTEST_TIMEOUT") . ok () . map (| to | quote ! { core :: time :: Duration :: from_secs ((# to) . parse () . unwrap ()) }) }) ; let rstest_path = crate_name () ; match (timeout , is_async) { (Some (to_expr) , true) => quote ! { use # rstest_path :: timeout ::*; execute_with_timeout_async (move || # fn_path (# (# args) ,*) , # to_expr) . await } , (Some (to_expr) , false) => quote ! { use # rstest_path :: timeout ::*; execute_with_timeout_sync (move || # fn_path (# (# args) ,*) , # to_expr) } , _ => render_exec_call (fn_path , args , is_async) , } }
};
}
