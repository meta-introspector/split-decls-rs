// Generated macro for take_until (function)
macro_rules! Depcrate_parser_repeattake_until {
() => {
// Module: crate::parser::repeat
// Provides: {"take_until"}
// Dependencies: {}
# [doc = " Takes input until `end` is encountered or `end` indicates that it has committed input before"] # [doc = " failing (`attempt` can be used to make it look like it has not committed any input)"] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # use combine::parser::char;"] # [doc = " # use combine::parser::byte;"] # [doc = " # use combine::parser::combinator::attempt;"] # [doc = " # use combine::parser::repeat::take_until;"] # [doc = " # fn main() {"] # [doc = " let mut char_parser = take_until(char::digit());"] # [doc = " assert_eq!(char_parser.parse(\"abc123\"), Ok((\"abc\".to_string(), \"123\")));"] # [doc = ""] # [doc = " let mut byte_parser = take_until(byte::bytes(&b\"TAG\"[..]));"] # [doc = " assert_eq!(byte_parser.parse(&b\"123TAG\"[..]), Ok((b\"123\".to_vec(), &b\"TAG\"[..])));"] # [doc = " assert!(byte_parser.parse(&b\"123TATAG\"[..]).is_err());"] # [doc = ""] # [doc = " // `attempt` must be used if the `end` should be consume input before failing"] # [doc = " let mut byte_parser = take_until(attempt(byte::bytes(&b\"TAG\"[..])));"] # [doc = " assert_eq!(byte_parser.parse(&b\"123TATAG\"[..]), Ok((b\"123TA\".to_vec(), &b\"TAG\"[..])));"] # [doc = " # }"] # [doc = " ```"] pub fn take_until < F , Input , P > (end : P) -> TakeUntil < F , P > where Input : Stream , F : Extend < < Input as StreamOnce > :: Token > + Default , P : Parser < Input > , { TakeUntil { end , _marker : PhantomData , } }
};
}
