// Generated macro for hex_digit1 (function)
macro_rules! Depcrate_character_streaminghex_digit1 {
() => {
// Module: crate::character::streaming
// Provides: {"hex_digit1"}
// Dependencies: {}
# [doc = " Recognizes one or more ASCII hexadecimal numerical characters: 0-9, A-F, a-f"] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,"] # [doc = " or if no terminating token is found (a non hexadecimal digit character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::streaming::hex_digit1;"] # [doc = " assert_eq!(hex_digit1::<_, (_, ErrorKind)>(\"21cZ\"), Ok((\"Z\", \"21c\")));"] # [doc = " assert_eq!(hex_digit1::<_, (_, ErrorKind)>(\"H2\"), Err(Err::Error((\"H2\", ErrorKind::HexDigit))));"] # [doc = " assert_eq!(hex_digit1::<_, (_, ErrorKind)>(\"\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] pub fn hex_digit1 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position1 (| item | ! item . is_hex_digit () , ErrorKind :: HexDigit) }
};
}
