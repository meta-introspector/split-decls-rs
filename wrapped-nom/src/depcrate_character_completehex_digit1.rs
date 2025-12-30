// Generated macro for hex_digit1 (function)
macro_rules! Depcrate_character_completehex_digit1 {
() => {
// Module: crate::character::complete
// Provides: {"hex_digit1"}
// Dependencies: {}
# [doc = " Recognizes one or more ASCII hexadecimal numerical characters: 0-9, A-F, a-f"] # [doc = ""] # [doc = " *Complete version*: Will return an error if there's not enough input data,"] # [doc = " or the whole input if no terminating token is found (a non hexadecimal digit character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, IResult, Needed};"] # [doc = " # use nom::character::complete::hex_digit1;"] # [doc = " fn parser(input: &str) -> IResult<&str, &str> {"] # [doc = "     hex_digit1(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"21cZ\"), Ok((\"Z\", \"21c\")));"] # [doc = " assert_eq!(parser(\"H2\"), Err(Err::Error(Error::new(\"H2\", ErrorKind::HexDigit))));"] # [doc = " assert_eq!(parser(\"\"), Err(Err::Error(Error::new(\"\", ErrorKind::HexDigit))));"] # [doc = " ```"] pub fn hex_digit1 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position1_complete (| item | ! item . is_hex_digit () , ErrorKind :: HexDigit) }
};
}
