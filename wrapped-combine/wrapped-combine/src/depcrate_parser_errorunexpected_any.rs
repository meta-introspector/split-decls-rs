// Generated macro for unexpected_any (function)
macro_rules! Depcrate_parser_errorunexpected_any {
() => {
// Module: crate::parser::error
// Provides: {"unexpected_any"}
// Dependencies: {}
# [doc = " Always fails with `message` as an unexpected error."] # [doc = " Never consumes any input."] # [doc = ""] # [doc = " May have anything as the output type but must be used such that the output type can inferred."] # [doc = " The `unexpected` parser can be used if the output type does not matter"] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # use combine::parser::error::unexpected_any;"] # [doc = " # use combine::error::StreamError;"] # [doc = " # fn main() {"] # [doc = " let result = token('b').or(unexpected_any(\"token\"))"] # [doc = "     .easy_parse(\"a\");"] # [doc = " assert!(result.is_err());"] # [doc = " assert!("] # [doc = "     result.err()"] # [doc = "         .unwrap()"] # [doc = "         .errors"] # [doc = "         .iter()"] # [doc = "         .any(|m| *m == StreamError::unexpected(\"token\"))"] # [doc = " );"] # [doc = " # }"] # [doc = " ```"] pub fn unexpected_any < Input , S , T > (message : S) -> Unexpected < Input , T , S > where Input : Stream , S : for < 's > ErrorInfo < 's , Input :: Token , Input :: Range > , { Unexpected (message , PhantomData) }
};
}
