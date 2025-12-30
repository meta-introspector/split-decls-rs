// Generated macro for oct_digit1 (function)
macro_rules! Depcrate_character_completeoct_digit1 {
() => {
// Module: crate::character::complete
// Provides: {"oct_digit1"}
// Dependencies: {}
# [doc = " Recognizes one or more octal characters: 0-7"] # [doc = ""] # [doc = " *Complete version*: Will return an error if there's not enough input data,"] # [doc = " or the whole input if no terminating token is found (a non octal digit character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, IResult, Needed};"] # [doc = " # use nom::character::complete::oct_digit1;"] # [doc = " fn parser(input: &str) -> IResult<&str, &str> {"] # [doc = "     oct_digit1(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"21cZ\"), Ok((\"cZ\", \"21\")));"] # [doc = " assert_eq!(parser(\"H2\"), Err(Err::Error(Error::new(\"H2\", ErrorKind::OctDigit))));"] # [doc = " assert_eq!(parser(\"\"), Err(Err::Error(Error::new(\"\", ErrorKind::OctDigit))));"] # [doc = " ```"] pub fn oct_digit1 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position1_complete (| item | ! item . is_oct_digit () , ErrorKind :: OctDigit) }
};
}
