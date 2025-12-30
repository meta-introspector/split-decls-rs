// Generated macro for digit1 (function)
macro_rules! Depcrate_character_streamingdigit1 {
() => {
// Module: crate::character::streaming
// Provides: {"digit1"}
// Dependencies: {}
# [doc = " Recognizes one or more ASCII numerical characters: 0-9"] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,"] # [doc = " or if no terminating token is found (a non digit character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::streaming::digit1;"] # [doc = " assert_eq!(digit1::<_, (_, ErrorKind)>(\"21c\"), Ok((\"c\", \"21\")));"] # [doc = " assert_eq!(digit1::<_, (_, ErrorKind)>(\"c1\"), Err(Err::Error((\"c1\", ErrorKind::Digit))));"] # [doc = " assert_eq!(digit1::<_, (_, ErrorKind)>(\"\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] pub fn digit1 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position1 (| item | ! item . is_dec_digit () , ErrorKind :: Digit) }
};
}
