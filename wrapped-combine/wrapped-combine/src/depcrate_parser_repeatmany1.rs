// Generated macro for many1 (function)
macro_rules! Depcrate_parser_repeatmany1 {
() => {
// Module: crate::parser::repeat
// Provides: {"many1"}
// Dependencies: {}
# [doc = " Parses `p` one or more times returning a collection with the values from `p`."] # [doc = ""] # [doc = " If the returned collection cannot be inferred type annotations must be supplied, either by"] # [doc = " annotating the resulting type binding `let collection: Vec<_> = ...` or by specializing when"] # [doc = " calling many1 `many1::<Vec<_>, _>(...)`."] # [doc = ""] # [doc = " NOTE: If `p` can succeed without consuming any input this may hang forever as `many1` will"] # [doc = " repeatedly use `p` to parse the same location in the input every time"] # [doc = ""] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # use combine::parser::char::digit;"] # [doc = " # fn main() {"] # [doc = " let result = many1::<Vec<_>, _, _>(digit())"] # [doc = "     .parse(\"A123\");"] # [doc = " assert!(result.is_err());"] # [doc = " # }"] # [doc = " ```"] pub fn many1 < F , Input , P > (p : P) -> Many1 < F , P > where Input : Stream , F : Extend < P :: Output > + Default , P : Parser < Input > , { Many1 (p , PhantomData) }
};
}
