macro_rules! deps {
    () => {
        OnlyOneArgument!();
    };
}

macro_rules! get_single_expr_from_tts {
    () => {
        deps!();
        # [doc = " Interpreting `tts` as a comma-separated sequence of expressions,"] # [doc = " expect exactly one expression, or emit an error and return `Err`."] pub (crate) fn get_single_expr_from_tts (cx : & mut ExtCtxt < '_ > , span : Span , tts : TokenStream , name : & str ,) -> ExpandResult < Result < Box < ast :: Expr > , ErrorGuaranteed > , () > { let mut p = cx . new_parser_from_tts (tts) ; if p . token == token :: Eof { let guar = cx . dcx () . emit_err (errors :: OnlyOneArgument { span , name }) ; return ExpandResult :: Ready (Err (guar)) ; } let ret = match parse_expr (& mut p) { Ok (ret) => ret , Err (guar) => return ExpandResult :: Ready (Err (guar)) , } ; let _ = p . eat (exp ! (Comma)) ; if p . token != token :: Eof { cx . dcx () . emit_err (errors :: OnlyOneArgument { span , name }) ; } ExpandResult :: Ready (Ok (ret)) }
    };
}

get_single_expr_from_tts!()