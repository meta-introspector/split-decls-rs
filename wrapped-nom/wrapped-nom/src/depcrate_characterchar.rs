// Generated macro for char (function)
macro_rules! Depcrate_characterchar {
() => {
// Module: crate::character
// Provides: {"char"}
// Dependencies: {}
# [doc = " Recognizes one character."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::{ErrorKind, Error}, Needed, IResult};"] # [doc = " # use nom::character::streaming::char;"] # [doc = " fn parser(i: &str) -> IResult<&str, char> {"] # [doc = "     char('a')(i)"] # [doc = " }"] # [doc = " assert_eq!(parser(\"abc\"), Ok((\"bc\", 'a')));"] # [doc = " assert_eq!(parser(\"bc\"), Err(Err::Error(Error::new(\"bc\", ErrorKind::Char))));"] # [doc = " assert_eq!(parser(\"\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] pub fn char < I , Error : ParseError < I > > (c : char) -> impl Parser < I , Output = char , Error = Error > where I : Input , < I as Input > :: Item : AsChar , { Char { c , e : PhantomData } }
};
}
