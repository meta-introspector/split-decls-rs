// Generated macro for one_of (function)
macro_rules! Depcrate_character_completeone_of {
() => {
// Module: crate::character::complete
// Provides: {"one_of"}
// Dependencies: {}
# [doc = " Recognizes one of the provided characters."] # [doc = ""] # [doc = " *Complete version*: Will return an error if there's not enough input data."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind};"] # [doc = " # use nom::character::complete::one_of;"] # [doc = " assert_eq!(one_of::<_, _, (&str, ErrorKind)>(\"abc\")(\"b\"), Ok((\"\", 'b')));"] # [doc = " assert_eq!(one_of::<_, _, (&str, ErrorKind)>(\"a\")(\"bc\"), Err(Err::Error((\"bc\", ErrorKind::OneOf))));"] # [doc = " assert_eq!(one_of::<_, _, (&str, ErrorKind)>(\"a\")(\"\"), Err(Err::Error((\"\", ErrorKind::OneOf))));"] # [doc = " ```"] pub fn one_of < I , T , Error : ParseError < I > > (list : T) -> impl FnMut (I) -> IResult < I , char , Error > where I : Input , < I as Input > :: Item : AsChar , T : FindToken < char > , { let mut parser = super :: one_of (list) ; move | i : I | parser . process :: < OutputM < Emit , Emit , Complete > > (i) }
};
}
