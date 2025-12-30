// Generated macro for one_of (function)
macro_rules! Depcrate_character_streamingone_of {
() => {
// Module: crate::character::streaming
// Provides: {"one_of"}
// Dependencies: {}
# [doc = " Recognizes one of the provided characters."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::character::streaming::one_of;"] # [doc = " assert_eq!(one_of::<_, _, (_, ErrorKind)>(\"abc\")(\"b\"), Ok((\"\", 'b')));"] # [doc = " assert_eq!(one_of::<_, _, (_, ErrorKind)>(\"a\")(\"bc\"), Err(Err::Error((\"bc\", ErrorKind::OneOf))));"] # [doc = " assert_eq!(one_of::<_, _, (_, ErrorKind)>(\"a\")(\"\"), Err(Err::Incomplete(Needed::Unknown)));"] # [doc = " ```"] pub fn one_of < I , T , Error : ParseError < I > > (list : T) -> impl FnMut (I) -> IResult < I , char , Error > where I : Input , < I as Input > :: Item : AsChar , T : FindToken < char > , { let mut parser = super :: one_of (list) ; move | i : I | parser . process :: < OutputM < Emit , Emit , Streaming > > (i) }
};
}
