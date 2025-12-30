// Generated macro for bin_digit1 (function)
macro_rules! Depcrate_character_streamingbin_digit1 {
() => {
// Module: crate::character::streaming
// Provides: {"bin_digit1"}
// Dependencies: {}
# [doc = " Recognizes one or more binary characters: 0-1"] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,"] # [doc = " or if no terminating token is found (a non binary digit character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::streaming::bin_digit1;"] # [doc = " assert_eq!(bin_digit1::<_, (_, ErrorKind)>(\"013a\"), Ok((\"3a\", \"01\")));"] # [doc = " assert_eq!(bin_digit1::<_, (_, ErrorKind)>(\"a013\"), Err(Err::Error((\"a013\", ErrorKind::BinDigit))));"] # [doc = " assert_eq!(bin_digit1::<_, (_, ErrorKind)>(\"\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] pub fn bin_digit1 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position1 (| item | ! item . is_bin_digit () , ErrorKind :: BinDigit) }
};
}
