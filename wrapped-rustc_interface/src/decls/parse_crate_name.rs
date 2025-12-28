macro_rules! parse_crate_name {
    () => {
        pub (crate) fn parse_crate_name (sess : & Session , attrs : & [ast :: Attribute] , emit_errors : ShouldEmit ,) -> Option < (Symbol , Span) > { let rustc_hir :: Attribute :: Parsed (AttributeKind :: CrateName { name , name_span , .. }) = AttributeParser :: parse_limited_should_emit (sess , & attrs , sym :: crate_name , DUMMY_SP , rustc_ast :: node_id :: CRATE_NODE_ID , None , emit_errors ,) ? else { unreachable ! ("crate_name is the only attr we could've parsed here") ; } ; Some ((name , name_span)) }
    };
}

parse_crate_name!();