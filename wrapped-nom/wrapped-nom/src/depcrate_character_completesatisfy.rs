// Generated macro for satisfy (function)
macro_rules! Depcrate_character_completesatisfy {
() => {
// Module: crate::character::complete
// Provides: {"satisfy"}
// Dependencies: {}
# [doc = " Recognizes one character and checks that it satisfies a predicate"] # [doc = ""] # [doc = " *Complete version*: Will return an error if there's not enough input data."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::{ErrorKind, Error}, Needed, IResult};"] # [doc = " # use nom::character::complete::satisfy;"] # [doc = " fn parser(i: &str) -> IResult<&str, char> {"] # [doc = "     satisfy(|c| c == 'a' || c == 'b')(i)"] # [doc = " }"] # [doc = " assert_eq!(parser(\"abc\"), Ok((\"bc\", 'a')));"] # [doc = " assert_eq!(parser(\"cd\"), Err(Err::Error(Error::new(\"cd\", ErrorKind::Satisfy))));"] # [doc = " assert_eq!(parser(\"\"), Err(Err::Error(Error::new(\"\", ErrorKind::Satisfy))));"] # [doc = " ```"] pub fn satisfy < F , I , Error : ParseError < I > > (predicate : F) -> impl FnMut (I) -> IResult < I , char , Error > where I : Input , < I as Input > :: Item : AsChar , F : Fn (char) -> bool , { let mut parser = super :: satisfy (predicate) ; move | i : I | parser . process :: < OutputM < Emit , Emit , Complete > > (i) }
};
}
