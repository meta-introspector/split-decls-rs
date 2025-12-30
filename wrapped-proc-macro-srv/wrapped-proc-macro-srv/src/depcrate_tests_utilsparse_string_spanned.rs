// Generated macro for parse_string_spanned (function)
macro_rules! Depcrate_tests_utilsparse_string_spanned {
() => {
// Module: crate::tests::utils
// Provides: {"parse_string_spanned"}
// Dependencies: {}
fn parse_string_spanned (anchor : SpanAnchor , call_site : SyntaxContext , src : & str ,) -> crate :: server_impl :: TokenStream < Span > { crate :: server_impl :: TokenStream :: with_subtree (crate :: server_impl :: TopSubtree (syntax_bridge :: parse_to_token_tree (span :: Edition :: CURRENT , anchor , call_site , src) . unwrap () . 0 . into_vec () ,)) }
};
}
