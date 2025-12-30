// Generated macro for alpha1 (function)
macro_rules! Depcrate_character_completealpha1 {
() => {
// Module: crate::character::complete
// Provides: {"alpha1"}
// Dependencies: {}
# [doc = " Recognizes one or more lowercase and uppercase ASCII alphabetic characters: a-z, A-Z"] # [doc = ""] # [doc = " *Complete version*: Will return an error if there's not enough input data,"] # [doc = " or the whole input if no terminating token is found  (a non alphabetic character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, IResult, Needed};"] # [doc = " # use nom::character::complete::alpha1;"] # [doc = " fn parser(input: &str) -> IResult<&str, &str> {"] # [doc = "     alpha1(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"aB1c\"), Ok((\"1c\", \"aB\")));"] # [doc = " assert_eq!(parser(\"1c\"), Err(Err::Error(Error::new(\"1c\", ErrorKind::Alpha))));"] # [doc = " assert_eq!(parser(\"\"), Err(Err::Error(Error::new(\"\", ErrorKind::Alpha))));"] # [doc = " ```"] pub fn alpha1 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position1_complete (| item | ! item . is_alpha () , ErrorKind :: Alpha) }
};
}
