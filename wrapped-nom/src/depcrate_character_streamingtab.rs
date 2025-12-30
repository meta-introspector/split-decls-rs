// Generated macro for tab (function)
macro_rules! Depcrate_character_streamingtab {
() => {
// Module: crate::character::streaming
// Provides: {"tab"}
// Dependencies: {}
# [doc = " Matches a tab character '\\t'."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::streaming::tab;"] # [doc = " assert_eq!(tab::<_, (_, ErrorKind)>(\"\\tc\"), Ok((\"c\", '\\t')));"] # [doc = " assert_eq!(tab::<_, (_, ErrorKind)>(\"\\r\\nc\"), Err(Err::Error((\"\\r\\nc\", ErrorKind::Char))));"] # [doc = " assert_eq!(tab::<_, (_, ErrorKind)>(\"\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] pub fn tab < I , Error : ParseError < I > > (input : I) -> IResult < I , char , Error > where I : Input , < I as Input > :: Item : AsChar , { char ('\t') (input) }
};
}
