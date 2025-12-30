// Generated macro for none_of (function)
macro_rules! Depcrate_character_completenone_of {
() => {
// Module: crate::character::complete
// Provides: {"none_of"}
// Dependencies: {}
# [doc = " Recognizes a character that is not in the provided characters."] # [doc = ""] # [doc = " *Complete version*: Will return an error if there's not enough input data."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind};"] # [doc = " # use nom::character::complete::none_of;"] # [doc = " assert_eq!(none_of::<_, _, (&str, ErrorKind)>(\"abc\")(\"z\"), Ok((\"\", 'z')));"] # [doc = " assert_eq!(none_of::<_, _, (&str, ErrorKind)>(\"ab\")(\"a\"), Err(Err::Error((\"a\", ErrorKind::NoneOf))));"] # [doc = " assert_eq!(none_of::<_, _, (&str, ErrorKind)>(\"a\")(\"\"), Err(Err::Error((\"\", ErrorKind::NoneOf))));"] # [doc = " ```"] pub fn none_of < I , T , Error : ParseError < I > > (list : T) -> impl FnMut (I) -> IResult < I , char , Error > where I : Input , < I as Input > :: Item : AsChar , T : FindToken < char > , { let mut parser = super :: none_of (list) ; move | i : I | parser . process :: < OutputM < Emit , Emit , Complete > > (i) }
};
}
