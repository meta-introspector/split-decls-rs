macro_rules! expand_requires_tts {
    () => {
        fn expand_requires_tts (ecx : & mut ExtCtxt < '_ > , attr_span : Span , annotation : TokenStream , annotated : TokenStream ,) -> Result < TokenStream , ErrorGuaranteed > { let feature_span = ecx . with_def_site_ctxt (attr_span) ; expand_contract_clause (ecx , attr_span , annotated , | new_tts | { new_tts . push_tree (TokenTree :: Token (token :: Token :: from_ast_ident (Ident :: new (kw :: ContractRequires , feature_span)) , Spacing :: Joint ,)) ; new_tts . push_tree (TokenTree :: Token (token :: Token :: new (token :: TokenKind :: OrOr , attr_span) , Spacing :: Alone ,)) ; new_tts . push_tree (TokenTree :: Delimited (DelimSpan :: from_single (attr_span) , DelimSpacing :: new (Spacing :: JointHidden , Spacing :: JointHidden) , token :: Delimiter :: Parenthesis , annotation ,)) ; Ok (()) }) }
    };
}

expand_requires_tts!()