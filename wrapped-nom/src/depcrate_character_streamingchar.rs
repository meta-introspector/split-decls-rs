// Generated macro for char (function)
macro_rules! Depcrate_character_streamingchar {
() => {
// Module: crate::character::streaming
// Provides: {"char"}
// Dependencies: {}
# [doc = " Recognizes one character."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::{ErrorKind, Error}, Needed, IResult};"] # [doc = " # use nom::character::streaming::char;"] # [doc = " fn parser(i: &str) -> IResult<&str, char> {"] # [doc = "     char('a')(i)"] # [doc = " }"] # [doc = " assert_eq!(parser(\"abc\"), Ok((\"bc\", 'a')));"] # [doc = " assert_eq!(parser(\"bc\"), Err(Err::Error(Error::new(\"bc\", ErrorKind::Char))));"] # [doc = " assert_eq!(parser(\"\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] pub fn char < I , Error : ParseError < I > > (c : char) -> impl FnMut (I) -> IResult < I , char , Error > where I : Input , < I as Input > :: Item : AsChar , { let mut parser = super :: char (c) ; move | i : I | parser . process :: < OutputM < Emit , Emit , Streaming > > (i) }
};
}
