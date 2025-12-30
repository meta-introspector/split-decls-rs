// Generated macro for bool (function)
macro_rules! Depcrate_bits_streamingbool {
() => {
// Module: crate::bits::streaming
// Provides: {"bool"}
// Dependencies: {}
# [doc = " Parses one specific bit as a bool."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " # use nom::bits::complete::bool;"] # [doc = " # use nom::IResult;"] # [doc = " # use nom::error::{Error, ErrorKind};"] # [doc = ""] # [doc = " fn parse(input: (&[u8], usize)) -> IResult<(&[u8], usize), bool> {"] # [doc = "     bool(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parse(([0b10000000].as_ref(), 0)), Ok((([0b10000000].as_ref(), 1), true)));"] # [doc = " assert_eq!(parse(([0b10000000].as_ref(), 1)), Ok((([0b10000000].as_ref(), 2), false)));"] # [doc = " ```"] pub fn bool < I , E : ParseError < (I , usize) > > (input : (I , usize)) -> IResult < (I , usize) , bool , E > where I : Input < Item = u8 > , { let (res , bit) : (_ , u32) = take (1usize) (input) ? ; Ok ((res , bit != 0)) }
};
}
