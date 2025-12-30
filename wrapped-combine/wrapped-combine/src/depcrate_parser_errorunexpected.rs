// Generated macro for unexpected (function)
macro_rules! Depcrate_parser_errorunexpected {
() => {
// Module: crate::parser::error
// Provides: {"unexpected"}
// Dependencies: {}
# [doc = " Always fails with `message` as an unexpected error."] # [doc = " Never consumes any input."] # [doc = ""] # [doc = " Has `()` the output type"] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # use combine::error::StreamError;"] # [doc = " # fn main() {"] # [doc = " let result = unexpected(\"token\")"] # [doc = "     .easy_parse(\"a\");"] # [doc = " assert!(result.is_err());"] # [doc = " assert!("] # [doc = "     result.err()"] # [doc = "         .unwrap()"] # [doc = "         .errors"] # [doc = "         .iter()"] # [doc = "         .any(|m| *m == StreamError::unexpected(\"token\"))"] # [doc = " );"] # [doc = " # }"] # [doc = " ```"] pub fn unexpected < Input , S > (message : S) -> Unexpected < Input , () , S > where Input : Stream , S : for < 's > ErrorInfo < 's , Input :: Token , Input :: Range > , { unexpected_any (message) }
};
}
