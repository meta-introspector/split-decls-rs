// Generated macro for space1 (function)
macro_rules! Depcrate_character_completespace1 {
() => {
// Module: crate::character::complete
// Provides: {"space1"}
// Dependencies: {}
# [doc = " Recognizes one or more spaces and tabs."] # [doc = ""] # [doc = " *Complete version*: Will return an error if there's not enough input data,"] # [doc = " or the whole input if no terminating token is found (a non space character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, IResult, Needed};"] # [doc = " # use nom::character::complete::space1;"] # [doc = " fn parser(input: &str) -> IResult<&str, &str> {"] # [doc = "     space1(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\" \\t21c\"), Ok((\"21c\", \" \\t\")));"] # [doc = " assert_eq!(parser(\"H2\"), Err(Err::Error(Error::new(\"H2\", ErrorKind::Space))));"] # [doc = " assert_eq!(parser(\"\"), Err(Err::Error(Error::new(\"\", ErrorKind::Space))));"] # [doc = " ```"] pub fn space1 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position1_complete (| item | { let c = item . as_char () ; ! (c == ' ' || c == '\t') } , ErrorKind :: Space ,) }
};
}
