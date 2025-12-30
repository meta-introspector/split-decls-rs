// Generated macro for anychar (function)
macro_rules! Depcrate_character_streaminganychar {
() => {
// Module: crate::character::streaming
// Provides: {"anychar"}
// Dependencies: {}
# [doc = " Matches one element as a character."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{character::streaming::anychar, Err, error::ErrorKind, IResult, Needed};"] # [doc = " assert_eq!(anychar::<_, (_, ErrorKind)>(\"abc\"), Ok((\"bc\",'a')));"] # [doc = " assert_eq!(anychar::<_, (_, ErrorKind)>(\"\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] pub fn anychar < T , E : ParseError < T > > (input : T) -> IResult < T , char , E > where T : Input , < T as Input > :: Item : AsChar , { let mut it = input . iter_elements () ; match it . next () { None => Err (Err :: Incomplete (Needed :: new (1))) , Some (c) => Ok ((input . take_from (c . len ()) , c . as_char ())) , } }
};
}
