macro_rules! deps {
    () => {
        TopSubtree!();
    };
}

macro_rules! parse_string {
    () => {
        deps!();
        fn parse_string (call_site : SpanId , src : & str) -> crate :: server_impl :: TokenStream < SpanId > { crate :: server_impl :: TokenStream :: with_subtree (crate :: server_impl :: TopSubtree (syntax_bridge :: parse_to_token_tree_static_span (span :: Edition :: CURRENT , call_site , src) . unwrap () . 0 . into_vec () ,)) }
    };
}

parse_string!()