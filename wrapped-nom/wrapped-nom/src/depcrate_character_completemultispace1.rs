// Generated macro for multispace1 (function)
macro_rules! Depcrate_character_completemultispace1 {
() => {
// Module: crate::character::complete
// Provides: {"multispace1"}
// Dependencies: {}
# [doc = " Recognizes one or more spaces, tabs, carriage returns and line feeds."] # [doc = ""] # [doc = " *Complete version*: will return an error if there's not enough input data,"] # [doc = " or the whole input if no terminating token is found (a non space character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, IResult, Needed};"] # [doc = " # use nom::character::complete::multispace1;"] # [doc = " fn parser(input: &str) -> IResult<&str, &str> {"] # [doc = "     multispace1(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\" \\t\\n\\r21c\"), Ok((\"21c\", \" \\t\\n\\r\")));"] # [doc = " assert_eq!(parser(\"H2\"), Err(Err::Error(Error::new(\"H2\", ErrorKind::MultiSpace))));"] # [doc = " assert_eq!(parser(\"\"), Err(Err::Error(Error::new(\"\", ErrorKind::MultiSpace))));"] # [doc = " ```"] pub fn multispace1 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position1_complete (| item | { let c = item . as_char () ; ! (c == ' ' || c == '\t' || c == '\r' || c == '\n') } , ErrorKind :: MultiSpace ,) }
};
}
