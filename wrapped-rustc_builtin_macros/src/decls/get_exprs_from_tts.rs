macro_rules! deps {
    () => {
        ExpectedCommaInList!();
    };
}

macro_rules! get_exprs_from_tts {
    () => {
        deps!();
        # [doc = " Extracts comma-separated expressions from `tts`."] # [doc = " On error, emit it, and return `Err`."] pub (crate) fn get_exprs_from_tts (cx : & mut ExtCtxt < '_ > , tts : TokenStream ,) -> ExpandResult < Result < Vec < Box < ast :: Expr > > , ErrorGuaranteed > , () > { let mut p = cx . new_parser_from_tts (tts) ; let mut es = Vec :: new () ; while p . token != token :: Eof { let expr = match parse_expr (& mut p) { Ok (expr) => expr , Err (guar) => return ExpandResult :: Ready (Err (guar)) , } ; if ! cx . force_mode && let ast :: ExprKind :: MacCall (m) = & expr . kind && cx . resolver . macro_accessible (cx . current_expansion . id , & m . path) . is_err () { return ExpandResult :: Retry (()) ; } let expr = cx . expander () . fully_expand_fragment (AstFragment :: Expr (expr)) . make_expr () ; es . push (expr) ; if p . eat (exp ! (Comma)) { continue ; } if p . token != token :: Eof { let guar = cx . dcx () . emit_err (errors :: ExpectedCommaInList { span : p . token . span }) ; return ExpandResult :: Ready (Err (guar)) ; } } ExpandResult :: Ready (Ok (es)) }
    };
}

get_exprs_from_tts!();