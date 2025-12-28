macro_rules! deps {
    () => {
        OneCfgPattern!();
        RequiresCfgPattern!();
    };
}

macro_rules! parse_cfg {
    () => {
        deps!();
        fn parse_cfg < 'a > (cx : & ExtCtxt < 'a > , span : Span , tts : TokenStream ,) -> PResult < 'a , ast :: MetaItemInner > { let mut p = cx . new_parser_from_tts (tts) ; if p . token == token :: Eof { return Err (cx . dcx () . create_err (errors :: RequiresCfgPattern { span })) ; } let cfg = p . parse_meta_item_inner () ? ; let _ = p . eat (exp ! (Comma)) ; if ! p . eat (exp ! (Eof)) { return Err (cx . dcx () . create_err (errors :: OneCfgPattern { span })) ; } Ok (cfg) }
    };
}

parse_cfg!();