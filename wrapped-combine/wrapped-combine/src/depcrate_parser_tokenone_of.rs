// Generated macro for one_of (function)
macro_rules! Depcrate_parser_tokenone_of {
() => {
// Module: crate::parser::token
// Provides: {"one_of"}
// Dependencies: {}
# [doc = " Extract one token and succeeds if it is part of `tokens`."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # fn main() {"] # [doc = " let result = many(one_of(\"abc\".chars()))"] # [doc = "     .parse(\"abd\");"] # [doc = " assert_eq!(result, Ok((String::from(\"ab\"), \"d\")));"] # [doc = " # }"] # [doc = " ```"] pub fn one_of < T , Input > (tokens : T) -> OneOf < T , Input > where T : Clone + IntoIterator , Input : Stream , Input :: Token : PartialEq < T :: Item > , { OneOf { tokens , _marker : PhantomData , } }
};
}
