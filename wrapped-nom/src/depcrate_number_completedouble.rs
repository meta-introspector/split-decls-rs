// Generated macro for double (function)
macro_rules! Depcrate_number_completedouble {
() => {
// Module: crate::number::complete
// Provides: {"double"}
// Dependencies: {}
# [doc = " Recognizes floating point number in text format and returns a f64."] # [doc = ""] # [doc = " *Complete version*: Can parse until the end of input."] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::Needed::Size;"] # [doc = " use nom::number::complete::double;"] # [doc = ""] # [doc = " let parser = |s| {"] # [doc = "   double(s)"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(parser(\"11e-1\"), Ok((\"\", 1.1)));"] # [doc = " assert_eq!(parser(\"123E-02\"), Ok((\"\", 1.23)));"] # [doc = " assert_eq!(parser(\"123K-01\"), Ok((\"K-01\", 123.0)));"] # [doc = " assert_eq!(parser(\"abc\"), Err(Err::Error((\"abc\", ErrorKind::Float))));"] # [doc = " ```"] pub fn double < T , E : ParseError < T > > (input : T) -> IResult < T , f64 , E > where T : Clone + Offset + ParseTo < f64 > + Compare < & 'static str > , T : Input , < T as Input > :: Item : AsChar , < T as Input > :: Iter : Clone , T : AsBytes , T : for < 'a > Compare < & 'a [u8] > , { let (i , s) = recognize_float_or_exceptions (input) ? ; match s . parse_to () { Some (f) => Ok ((i , f)) , None => Err (crate :: Err :: Error (E :: from_error_kind (i , crate :: error :: ErrorKind :: Float ,))) , } }
};
}
