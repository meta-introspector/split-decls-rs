// Generated macro for line_ending (function)
macro_rules! Depcrate_character_completeline_ending {
() => {
// Module: crate::character::complete
// Provides: {"line_ending"}
// Dependencies: {}
# [doc = " Recognizes an end of line (both '\\n' and '\\r\\n')."] # [doc = ""] # [doc = " *Complete version*: Will return an error if there's not enough input data."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::{Error, ErrorKind}, IResult, Needed};"] # [doc = " # use nom::character::complete::line_ending;"] # [doc = " fn parser(input: &str) -> IResult<&str, &str> {"] # [doc = "     line_ending(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser(\"\\r\\nc\"), Ok((\"c\", \"\\r\\n\")));"] # [doc = " assert_eq!(parser(\"ab\\r\\nc\"), Err(Err::Error(Error::new(\"ab\\r\\nc\", ErrorKind::CrLf))));"] # [doc = " assert_eq!(parser(\"\"), Err(Err::Error(Error::new(\"\", ErrorKind::CrLf))));"] # [doc = " ```"] pub fn line_ending < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , T : Compare < & 'static str > , { match input . compare ("\n") { CompareResult :: Ok => Ok (input . take_split (1)) , CompareResult :: Incomplete => Err (Err :: Error (E :: from_error_kind (input , ErrorKind :: CrLf))) , CompareResult :: Error => match input . compare ("\r\n") { CompareResult :: Ok => Ok (input . take_split (2)) , _ => Err (Err :: Error (E :: from_error_kind (input , ErrorKind :: CrLf))) , } , } }
};
}
