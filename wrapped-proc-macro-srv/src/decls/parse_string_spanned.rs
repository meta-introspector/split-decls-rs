macro_rules! deps {
    () => {
        TopSubtree!();
    };
}

macro_rules! parse_string_spanned {
    () => {
        deps!();
        fn parse_string_spanned (anchor : SpanAnchor , call_site : SyntaxContext , src : & str ,) -> crate :: server_impl :: TokenStream < Span > { crate :: server_impl :: TokenStream :: with_subtree (crate :: server_impl :: TopSubtree (syntax_bridge :: parse_to_token_tree (span :: Edition :: CURRENT , anchor , call_site , src) . unwrap () . 0 . into_vec () ,)) }
    };
}

parse_string_spanned!()