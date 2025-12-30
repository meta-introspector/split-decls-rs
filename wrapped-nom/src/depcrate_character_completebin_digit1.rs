// Generated macro for bin_digit1 (function)
macro_rules! Depcrate_character_completebin_digit1 {
() => {
// Module: crate::character::complete
// Provides: {"bin_digit1"}
// Dependencies: {}
# [doc = " Recognizes one or more binary characters: 0-1"] # [doc = ""] # [doc = " *Complete version*: Will return an error if there's not enough input data,"] # [doc = " or the whole input if no terminating token is found (a non binary digit character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, IResult, Needed};"] # [doc = " # use nom::character::complete::bin_digit1;"] # [doc = " fn parser(input: &str) -> IResult<&str, &str> {"] # [doc = "     bin_digit1(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"013a\"), Ok((\"3a\", \"01\")));"] # [doc = " assert_eq!(parser(\"a013\"), Err(Err::Error(Error::new(\"a013\", ErrorKind::BinDigit))));"] # [doc = " assert_eq!(parser(\"\"), Err(Err::Error(Error::new(\"\", ErrorKind::BinDigit))));"] # [doc = " ```"] pub fn bin_digit1 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position1_complete (| item | ! item . is_bin_digit () , ErrorKind :: BinDigit) }
};
}
