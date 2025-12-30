// Generated macro for eof (function)
macro_rules! Depcrate_combinatoreof {
() => {
// Module: crate::combinator
// Provides: {"eof"}
// Dependencies: {}
# [doc = " returns its input if it is at the end of input data"] # [doc = ""] # [doc = " When we're at the end of the data, this combinator"] # [doc = " will succeed"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::str;"] # [doc = " # use nom::{Err, error::ErrorKind, IResult};"] # [doc = " # use nom::combinator::eof;"] # [doc = ""] # [doc = " # fn main() {"] # [doc = " let parser = eof;"] # [doc = " assert_eq!(parser(\"abc\"), Err(Err::Error((\"abc\", ErrorKind::Eof))));"] # [doc = " assert_eq!(parser(\"\"), Ok((\"\", \"\")));"] # [doc = " # }"] # [doc = " ```"] pub fn eof < I : Input + Clone , E : ParseError < I > > (input : I) -> IResult < I , I , E > { if input . input_len () == 0 { let clone = input . clone () ; Ok ((input , clone)) } else { Err (Err :: Error (E :: from_error_kind (input , ErrorKind :: Eof))) } }
};
}
