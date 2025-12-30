// Generated macro for any (function)
macro_rules! Depcrate_parser_tokenany {
() => {
// Module: crate::parser::token
// Provides: {"any"}
// Dependencies: {}
# [doc = " Parses any token."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # fn main() {"] # [doc = " let mut char_parser = any();"] # [doc = " assert_eq!(char_parser.parse(\"!\").map(|x| x.0), Ok('!'));"] # [doc = " assert!(char_parser.parse(\"\").is_err());"] # [doc = " let mut byte_parser = any();"] # [doc = " assert_eq!(byte_parser.parse(&b\"!\"[..]).map(|x| x.0), Ok(b'!'));"] # [doc = " assert!(byte_parser.parse(&b\"\"[..]).is_err());"] # [doc = " # }"] # [doc = " ```"] pub fn any < Input > () -> Any < Input > where Input : Stream , { Any (PhantomData) }
};
}
