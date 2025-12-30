// Generated macro for crlf (function)
macro_rules! Depcrate_character_completecrlf {
() => {
// Module: crate::character::complete
// Provides: {"crlf"}
// Dependencies: {}
# [doc = " Recognizes the string \"\\r\\n\"."] # [doc = ""] # [doc = " *Complete version*: Will return an error if there's not enough input data."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, IResult};"] # [doc = " # use nom::character::complete::crlf;"] # [doc = " fn parser(input: &str) -> IResult<&str, &str> {"] # [doc = "     crlf(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"\\r\\nc\"), Ok((\"c\", \"\\r\\n\")));"] # [doc = " assert_eq!(parser(\"ab\\r\\nc\"), Err(Err::Error(Error::new(\"ab\\r\\nc\", ErrorKind::CrLf))));"] # [doc = " assert_eq!(parser(\"\"), Err(Err::Error(Error::new(\"\", ErrorKind::CrLf))));"] # [doc = " ```"] pub fn crlf < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , T : Compare < & 'static str > , { match input . compare ("\r\n") { CompareResult :: Ok => Ok (input . take_split (2)) , _ => { let e : ErrorKind = ErrorKind :: CrLf ; Err (Err :: Error (E :: from_error_kind (input , e))) } } }
};
}
