// Generated macro for parse_string (function)
macro_rules! Depcrate_tests_utilsparse_string {
() => {
// Module: crate::tests::utils
// Provides: {"parse_string"}
// Dependencies: {}
fn parse_string (call_site : SpanId , src : & str) -> crate :: server_impl :: TokenStream < SpanId > { crate :: server_impl :: TokenStream :: with_subtree (crate :: server_impl :: TopSubtree (syntax_bridge :: parse_to_token_tree_static_span (span :: Edition :: CURRENT , call_site , src) . unwrap () . 0 . into_vec () ,)) }
};
}
