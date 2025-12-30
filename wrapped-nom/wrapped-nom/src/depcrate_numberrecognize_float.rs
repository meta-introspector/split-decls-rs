// Generated macro for recognize_float (function)
macro_rules! Depcrate_numberrecognize_float {
() => {
// Module: crate::number
// Provides: {"recognize_float"}
// Dependencies: {}
# [doc = " Recognizes a floating point number in text format and returns the corresponding part of the input."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if it reaches the end of input."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, Parser};"] # [doc = " use nom::number::recognize_float;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   recognize_float().parse(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(\"11e-1;\"), Ok((\";\", \"11e-1\")));"] # [doc = " assert_eq!(parser(\"123E-02;\"), Ok((\";\", \"123E-02\")));"] # [doc = " assert_eq!(parser(\"123K-01\"), Ok((\"K-01\", \"123\")));"] # [doc = " assert_eq!(parser(\"abc\"), Err(Err::Error((\"abc\", ErrorKind::Char))));"] # [doc = " ```"] # [rustfmt :: skip] pub fn recognize_float < T , E : ParseError < T > > () -> impl Parser < T , Output = T , Error = E > where T : Clone + Offset , T : Input , < T as Input > :: Item : AsChar , { recognize ((opt (alt ((char ('+') , char ('-')))) , alt ((map ((digit1 () , opt (pair (char ('.') , opt (digit1 ())))) , | _ | ()) , map ((char ('.') , digit1 ()) , | _ | ()))) , opt ((alt ((char ('e') , char ('E'))) , opt (alt ((char ('+') , char ('-')))) , cut (digit1 ()))))) }
};
}
