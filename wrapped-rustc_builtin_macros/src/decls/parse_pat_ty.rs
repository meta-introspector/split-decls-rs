macro_rules! deps {
    () => {
        Ty!();
    };
}

macro_rules! parse_pat_ty {
    () => {
        deps!();
        fn parse_pat_ty < 'a > (cx : & mut ExtCtxt < 'a > , stream : TokenStream ,) -> PResult < 'a , (Box < Ty > , Box < TyPat >) > { let mut parser = cx . new_parser_from_tts (stream) ; let ty = parser . parse_ty () ? ; parser . expect_keyword (exp ! (Is)) ? ; let pat = pat_to_ty_pat (cx , * parser . parse_pat_no_top_guard (None , RecoverComma :: No , RecoverColon :: No , CommaRecoveryMode :: EitherTupleOrPipe ,) ? ,) ; if parser . token != token :: Eof { parser . unexpected () ? ; } Ok ((ty , pat)) }
    };
}

parse_pat_ty!()