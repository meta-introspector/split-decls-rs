// Generated macro for parse_description (function)
macro_rules! Depcrate_parser_documentparse_description {
() => {
// Module: crate::parser::document
// Provides: {"parse_description"}
// Dependencies: {}
fn parse_description < 'a > (parser : & mut Parser < 'a >) -> OptionParseResult < Cow < 'a , str > > { if ! matches ! (parser . peek () . item , Token :: Scalar (ScalarToken :: String (_))) { Ok (None) } else { let token = parser . next_token () ? ; let Token :: Scalar (ScalarToken :: String (lit)) = token . item else { unreachable ! ("already checked to be `ScalarToken::String`") } ; Ok (Some (Spanning :: new (token . span , lit . parse () . map_err (| e | Spanning :: new (token . span , e)) ? ,))) } }
};
}
