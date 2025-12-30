// Generated macro for alphanumeric1 (function)
macro_rules! Depcrate_character_streamingalphanumeric1 {
() => {
// Module: crate::character::streaming
// Provides: {"alphanumeric1"}
// Dependencies: {}
# [doc = " Recognizes one or more ASCII numerical and alphabetic characters: 0-9, a-z, A-Z"] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,"] # [doc = " or if no terminating token is found (a non alphanumerical character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::streaming::alphanumeric1;"] # [doc = " assert_eq!(alphanumeric1::<_, (_, ErrorKind)>(\"21cZ%1\"), Ok((\"%1\", \"21cZ\")));"] # [doc = " assert_eq!(alphanumeric1::<_, (_, ErrorKind)>(\"&H2\"), Err(Err::Error((\"&H2\", ErrorKind::AlphaNumeric))));"] # [doc = " assert_eq!(alphanumeric1::<_, (_, ErrorKind)>(\"\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] pub fn alphanumeric1 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position1 (| item | ! item . is_alphanum () , ErrorKind :: AlphaNumeric) }
};
}
