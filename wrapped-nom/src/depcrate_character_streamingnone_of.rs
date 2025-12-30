// Generated macro for none_of (function)
macro_rules! Depcrate_character_streamingnone_of {
() => {
// Module: crate::character::streaming
// Provides: {"none_of"}
// Dependencies: {}
# [doc = " Recognizes a character that is not in the provided characters."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, Needed};"] # [doc = " # use nom::character::streaming::none_of;"] # [doc = " assert_eq!(none_of::<_, _, (_, ErrorKind)>(\"abc\")(\"z\"), Ok((\"\", 'z')));"] # [doc = " assert_eq!(none_of::<_, _, (_, ErrorKind)>(\"ab\")(\"a\"), Err(Err::Error((\"a\", ErrorKind::NoneOf))));"] # [doc = " assert_eq!(none_of::<_, _, (_, ErrorKind)>(\"a\")(\"\"), Err(Err::Incomplete(Needed::Unknown)));"] # [doc = " ```"] pub fn none_of < I , T , Error : ParseError < I > > (list : T) -> impl FnMut (I) -> IResult < I , char , Error > where I : Input , < I as Input > :: Item : AsChar , T : FindToken < char > , { let mut parser = super :: none_of (list) ; move | i : I | parser . process :: < OutputM < Emit , Emit , Streaming > > (i) }
};
}
