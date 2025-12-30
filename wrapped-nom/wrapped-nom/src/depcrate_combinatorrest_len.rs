// Generated macro for rest_len (function)
macro_rules! Depcrate_combinatorrest_len {
() => {
// Module: crate::combinator
// Provides: {"rest_len"}
// Dependencies: {}
# [doc = " Return the length of the remaining input."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use nom::error::ErrorKind;"] # [doc = " use nom::combinator::rest_len;"] # [doc = " assert_eq!(rest_len::<_,(_, ErrorKind)>(\"abc\"), Ok((\"abc\", 3)));"] # [doc = " assert_eq!(rest_len::<_,(_, ErrorKind)>(\"\"), Ok((\"\", 0)));"] # [doc = " ```"] # [inline] pub fn rest_len < T , E : ParseError < T > > (input : T) -> IResult < T , usize , E > where T : Input , { let len = input . input_len () ; Ok ((input , len)) }
};
}
