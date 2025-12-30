// Generated macro for parse_tag (function)
macro_rules! Depcrate_match_tokenparse_tag {
() => {
// Module: crate::match_token
// Provides: {"parse_tag"}
// Dependencies: {}
fn parse_tag (parser : & mut Parser) -> Result < Spanned < Tag > , FatalError > { let lo = parser . span . lo ; try ! (parser . expect (& token :: Lt)) ; let kind = match try ! (parser . eat (& token :: BinOp (token :: Slash))) { true => EndTag , false => StartTag , } ; let name = match try ! (parser . eat (& token :: Underscore)) { true => None , false => Some ((* try ! (parser . parse_ident ()) . name . as_str ()) . to_owned ()) , } ; try ! (parser . expect (& token :: Gt)) ; Ok (spanned (lo , parser . last_span . hi , Tag { kind : kind , name : name , })) }
};
}
