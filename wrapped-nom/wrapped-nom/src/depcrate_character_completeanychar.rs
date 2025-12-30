// Generated macro for anychar (function)
macro_rules! Depcrate_character_completeanychar {
() => {
// Module: crate::character::complete
// Provides: {"anychar"}
// Dependencies: {}
# [doc = " Matches one byte as a character. Note that the input type will"] # [doc = " accept a `str`, but not a `&[u8]`, unlike many other nom parsers."] # [doc = ""] # [doc = " *Complete version*: Will return an error if there's not enough input data."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{character::complete::anychar, Err, error::{Error, ErrorKind}, IResult};"] # [doc = " fn parser(input: &str) -> IResult<&str, char> {"] # [doc = "     anychar(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"abc\"), Ok((\"bc\",'a')));"] # [doc = " assert_eq!(parser(\"\"), Err(Err::Error(Error::new(\"\", ErrorKind::Eof))));"] # [doc = " ```"] pub fn anychar < T , E : ParseError < T > > (input : T) -> IResult < T , char , E > where T : Input , < T as Input > :: Item : AsChar , { let mut it = input . iter_elements () ; match it . next () { None => Err (Err :: Error (E :: from_error_kind (input , ErrorKind :: Eof))) , Some (c) => Ok ((input . take_from (c . len ()) , c . as_char ())) , } }
};
}
