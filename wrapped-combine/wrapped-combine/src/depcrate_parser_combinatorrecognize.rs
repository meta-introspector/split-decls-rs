// Generated macro for recognize (function)
macro_rules! Depcrate_parser_combinatorrecognize {
() => {
// Module: crate::parser::combinator
// Provides: {"recognize"}
// Dependencies: {}
# [doc = " Constructs a parser which returns the tokens parsed by `parser` accumulated in"] # [doc = " `F: Extend<Input::Token>` instead of `P::Output`."] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::{repeat::skip_many1, token::token, combinator::recognize, char::digit};"] # [doc = ""] # [doc = " let mut parser = recognize((skip_many1(digit()), token('.'), skip_many1(digit())));"] # [doc = " assert_eq!(parser.parse(\"123.45\"), Ok((\"123.45\".to_string(), \"\")));"] # [doc = " assert_eq!(parser.parse(\"123.45\"), Ok((\"123.45\".to_string(), \"\")));"] # [doc = " ```"] pub fn recognize < F , Input , P > (parser : P) -> Recognize < F , P > where Input : Stream , P : Parser < Input > , F : Default + Extend < < Input as StreamOnce > :: Token > , { Recognize (parser , PhantomData) }
};
}
