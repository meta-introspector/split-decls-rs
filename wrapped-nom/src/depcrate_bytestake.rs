// Generated macro for take (function)
macro_rules! Depcrate_bytestake {
() => {
// Module: crate::bytes
// Provides: {"take"}
// Dependencies: {}
# [doc = " Returns an input slice containing the first N input elements (Input[..N])."] # [doc = ""] # [doc = " # Streaming Specific"] # [doc = " *Streaming version* if the input has less than N elements, `take` will"] # [doc = " return a `Err::Incomplete(Needed::new(M))` where M is the number of"] # [doc = " additional bytes the parser would need to succeed."] # [doc = " It is well defined for `&[u8]` as the number of elements is the byte size,"] # [doc = " but for types like `&str`, we cannot know how many bytes correspond for"] # [doc = " the next few chars, so the result will be `Err::Incomplete(Needed::Unknown)`"] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, IResult};"] # [doc = " use nom::bytes::streaming::take;"] # [doc = ""] # [doc = " fn take6(s: &str) -> IResult<&str, &str> {"] # [doc = "   take(6usize)(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(take6(\"1234567\"), Ok((\"7\", \"123456\")));"] # [doc = " assert_eq!(take6(\"things\"), Ok((\"\", \"things\")));"] # [doc = " assert_eq!(take6(\"short\"), Err(Err::Incomplete(Needed::Unknown)));"] # [doc = " ```"] pub fn take < C , I , Error : ParseError < I > > (count : C) -> impl Parser < I , Output = I , Error = Error > where I : Input , C : ToUsize , { Take { length : count . to_usize () , e : PhantomData , } }
};
}
