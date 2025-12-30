// Generated macro for parse_placeholder (function)
macro_rules! Depcrate_parsingparse_placeholder {
() => {
// Module: crate::parsing
// Provides: {"parse_placeholder"}
// Dependencies: {}
fn parse_placeholder (tokens : & mut std :: vec :: IntoIter < Token >) -> Result < Placeholder , SsrError > { let mut name = None ; let mut constraints = Vec :: new () ; if let Some (token) = tokens . next () { match token . kind { SyntaxKind :: IDENT => { name = Some (token . text) ; } T ! ['{'] => { let token = tokens . next () . ok_or_else (| | SsrError :: new ("Unexpected end of placeholder")) ? ; if token . kind == SyntaxKind :: IDENT { name = Some (token . text) ; } loop { let token = tokens . next () . ok_or_else (| | SsrError :: new ("Placeholder is missing closing brace '}'")) ? ; match token . kind { T ! [:] => { constraints . push (parse_constraint (tokens) ?) ; } T ! ['}'] => break , _ => bail ! ("Unexpected token while parsing placeholder: '{}'" , token . text) , } } } _ => { bail ! ("Placeholders should either be $name or ${{name:constraints}}") ; } } } let name = name . ok_or_else (| | SsrError :: new ("Placeholder ($) with no name")) ? ; Ok (Placeholder :: new (name , constraints)) }
};
}
