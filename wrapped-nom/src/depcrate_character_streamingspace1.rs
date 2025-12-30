// Generated macro for space1 (function)
macro_rules! Depcrate_character_streamingspace1 {
() => {
// Module: crate::character::streaming
// Provides: {"space1"}
// Dependencies: {}
# [doc = " Recognizes one or more spaces and tabs."] # [doc = ""] # [doc = " *Streaming version*: Will return `Err(nom::Err::Incomplete(_))` if there's not enough input data,"] # [doc = " or if no terminating token is found (a non space character)."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, IResult, Needed};"] # [doc = " # use nom::character::streaming::space1;"] # [doc = " assert_eq!(space1::<_, (_, ErrorKind)>(\" \\t21c\"), Ok((\"21c\", \" \\t\")));"] # [doc = " assert_eq!(space1::<_, (_, ErrorKind)>(\"H2\"), Err(Err::Error((\"H2\", ErrorKind::Space))));"] # [doc = " assert_eq!(space1::<_, (_, ErrorKind)>(\"\"), Err(Err::Incomplete(Needed::new(1))));"] # [doc = " ```"] pub fn space1 < T , E : ParseError < T > > (input : T) -> IResult < T , T , E > where T : Input , < T as Input > :: Item : AsChar , { input . split_at_position1 (| item | { let c = item . as_char () ; ! (c == ' ' || c == '\t') } , ErrorKind :: Space ,) }
};
}
