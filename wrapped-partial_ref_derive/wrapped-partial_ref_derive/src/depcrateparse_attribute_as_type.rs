// Generated macro for parse_attribute_as_type (function)
macro_rules! Depcrateparse_attribute_as_type {
() => {
// Module: crate
// Provides: {"parse_attribute_as_type"}
// Dependencies: {}
fn parse_attribute_as_type (attr : & Attribute) -> Type { if let Some (TokenTree :: Group (group)) = attr . tokens . clone () . into_iter () . next () { let parsed_type : Type = parse_quote ! (# group) ; if let Type :: Paren (TypeParen { elem , .. }) = parsed_type { return * elem ; } return parsed_type ; } let parse_panic = | | panic ! ("could not parse attribute `{}`" , attr . tokens . to_string ()) ; let meta = attr . parse_meta () . unwrap_or_else (| _ | parse_panic ()) ; if let Meta :: NameValue (name_value) = meta { if let Lit :: Str (string) = name_value . lit { match parse_str (& string . value ()) { Err (_) => panic ! ("could not parse type `{}` in attribute" , string . value ()) , Ok (parsed_type) => return parsed_type , } } } parse_panic () ; unreachable ! () }
};
}
