// Generated macro for parse_pat_ty (function)
macro_rules! Depcrate_pattern_typeparse_pat_ty {
() => {
// Module: crate::pattern_type
// Provides: {"parse_pat_ty"}
// Dependencies: {}
fn parse_pat_ty < 'a > (cx : & mut ExtCtxt < 'a > , stream : TokenStream ,) -> PResult < 'a , (Box < Ty > , Box < TyPat >) > { let mut parser = cx . new_parser_from_tts (stream) ; let ty = parser . parse_ty () ? ; parser . expect_keyword (exp ! (Is)) ? ; let pat = pat_to_ty_pat (cx , * parser . parse_pat_no_top_guard (None , RecoverComma :: No , RecoverColon :: No , CommaRecoveryMode :: EitherTupleOrPipe ,) ? ,) ; if parser . token != token :: Eof { parser . unexpected () ? ; } Ok ((ty , pat)) }
};
}
