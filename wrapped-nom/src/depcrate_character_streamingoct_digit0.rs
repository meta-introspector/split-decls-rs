// Generated macro for oct_digit0 (function)
macro_rules! Depcrate_character_streamingoct_digit0 {
() => {
// Module: crate::character::streaming
// Provides: {"oct_digit0"}
// Dependencies: {}
# [doc = " Recognizes zero or more octal characters: 0-7"] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,"] # [doc = " or if no terminating token is found (a non octal digit character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::streaming::oct_digit0;"] # [doc = " assert_eq!(oct_digit0::<_, (_, ErrorKind)>(\"21cZ\"), Ok((\"cZ\", \"21\")));"] # [doc = " assert_eq!(oct_digit0::<_, (_, ErrorKind)>(\"Z21c\"), Ok((\"Z21c\", \"\")));"] # [doc = " assert_eq!(oct_digit0::<_, (_, ErrorKind)>(\"\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] pub fn oct_digit0 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position (| item | ! item . is_oct_digit ()) }
};
}
