// Generated macro for tokens_cmp (function)
macro_rules! Depcrate_parser_tokentokens_cmp {
() => {
// Module: crate::parser::token
// Provides: {"tokens_cmp"}
// Dependencies: {}
# [doc = " Parses multiple tokens."] # [doc = ""] # [doc = " Consumes items from the input and compares them to the values from `tokens` using the"] # [doc = " comparison function `cmp`. Succeeds if all the items from `tokens` are matched in the input"] # [doc = " stream and fails otherwise."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # fn main() {"] # [doc = " # #[allow(deprecated)]"] # [doc = " # use std::ascii::AsciiExt;"] # [doc = " let result = tokens_cmp(\"abc\".chars(), |l, r| l.eq_ignore_ascii_case(&r))"] # [doc = "     .parse(\"AbC\")"] # [doc = "     .map(|x| x.0.as_str());"] # [doc = " assert_eq!(result, Ok(\"abc\"));"] # [doc = " let result = tokens_cmp("] # [doc = "     &b\"025\"[..],"] # [doc = "     |&l, r| (if l < r { r - l } else { l - r }) <= 2,"] # [doc = " )"] # [doc = "     .parse(&b\"123\"[..])"] # [doc = "     .map(|x| x.0);"] # [doc = " assert_eq!(result, Ok(&b\"025\"[..]));"] # [doc = " # }"] # [doc = " ```"] pub fn tokens_cmp < C , T , I > (tokens : T , cmp : C) -> TokensCmp < C , T , I > where C : FnMut (T :: Item , I :: Token) -> bool , T : Clone + IntoIterator , I : Stream , { TokensCmp { cmp , tokens , _marker : PhantomData , } }
};
}
