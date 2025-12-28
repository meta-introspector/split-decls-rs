macro_rules! deps {
    () => {
        AssertMissingComma!();
        AssertRequiresExpression!();
        AssertRequiresBoolean!();
        Assert!();
    };
}

macro_rules! parse_assert {
    () => {
        deps!();
        fn parse_assert < 'a > (cx : & ExtCtxt < 'a > , sp : Span , stream : TokenStream) -> PResult < 'a , Assert > { let mut parser = cx . new_parser_from_tts (stream) ; if parser . token == token :: Eof { return Err (cx . dcx () . create_err (errors :: AssertRequiresBoolean { span : sp })) ; } let cond_expr = parser . parse_expr () ? ; if parser . token == token :: Semi { cx . dcx () . emit_err (errors :: AssertRequiresExpression { span : sp , token : parser . token . span }) ; parser . bump () ; } let custom_message = if let token :: Literal (token :: Lit { kind : token :: Str , .. }) = parser . token . kind { let comma = parser . prev_token . span . shrink_to_hi () ; cx . dcx () . emit_err (errors :: AssertMissingComma { span : parser . token . span , comma }) ; parse_custom_message (& mut parser) } else if parser . eat (exp ! (Comma)) { parse_custom_message (& mut parser) } else { None } ; if parser . token != token :: Eof { parser . unexpected () ? ; } Ok (Assert { cond_expr , custom_message }) }
    };
}

parse_assert!();