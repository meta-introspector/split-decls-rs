macro_rules! deps {
    () => {
        Safety!();
        AttrArgs!();
        Spacing!();
        AttrIdGenerator!();
        Path!();
        TokenStream!();
        AttrStyle!();
        DelimSpan!();
        Delimiter!();
        Token!();
        DelimArgs!();
        TokenTree!();
        Attribute!();
    };
}

macro_rules! mk_attr_nested_word {
    () => {
        deps!();
        pub fn mk_attr_nested_word (g : & AttrIdGenerator , style : AttrStyle , unsafety : Safety , outer : Symbol , inner : Symbol , span : Span ,) -> Attribute { let inner_tokens = TokenStream :: new (vec ! [TokenTree :: Token (Token :: from_ast_ident (Ident :: new (inner , span)) , Spacing :: Alone ,)]) ; let outer_ident = Ident :: new (outer , span) ; let path = Path :: from_ident (outer_ident) ; let attr_args = AttrArgs :: Delimited (DelimArgs { dspan : DelimSpan :: from_single (span) , delim : Delimiter :: Parenthesis , tokens : inner_tokens , }) ; mk_attr (g , style , unsafety , path , attr_args , span) }
    };
}

mk_attr_nested_word!();