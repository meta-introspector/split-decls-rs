// Generated macro for pretty (function)
macro_rules! Depcrate_pre_expandpretty {
() => {
// Module: crate::pre_expand
// Provides: {"pretty"}
// Dependencies: {}
# [doc = " Somehow, going through a parser and back to tokens gives nicer whitespace."] fn pretty (cx : & mut ext :: base :: ExtCtxt , tts : Vec < ast :: TokenTree >) -> Vec < ast :: TokenTree > { let mut parser = parse :: new_parser_from_tts (cx . parse_sess () , cx . cfg () , tts) ; let start_span = parser . span ; let mut items = Vec :: new () ; let attrs = parser . parse_inner_attributes () . unwrap () ; while let Ok (Some (item)) = parser . parse_item () { items . push (item) } cx . bt_push (expn_info (start_span)) ; quote_tokens ! (& mut * cx , $ attrs $ items) }
};
}
