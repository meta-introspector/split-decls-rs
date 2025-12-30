// Generated macro for sep_end_by (function)
macro_rules! Depcrate_parser_repeatsep_end_by {
() => {
// Module: crate::parser::repeat
// Provides: {"sep_end_by"}
// Dependencies: {}
# [doc = " Parses `parser` zero or more times separated and ended by `separator`, returning a collection"] # [doc = " with the values from `p`."] # [doc = ""] # [doc = " If the returned collection cannot be inferred type annotations must be supplied, either by"] # [doc = " annotating the resulting type binding `let collection: Vec<_> = ...` or by specializing when"] # [doc = " calling `sep_by`, `sep_by::<Vec<_>, _, _>(...)`"] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # use combine::parser::char::digit;"] # [doc = " # fn main() {"] # [doc = " let mut parser = sep_end_by(digit(), token(';'));"] # [doc = " let result_ok = parser.parse(\"1;2;3;\");"] # [doc = " assert_eq!(result_ok, Ok((vec!['1', '2', '3'], \"\")));"] # [doc = " let result_ok2 = parser.parse(\"1;2;3\");"] # [doc = " assert_eq!(result_ok2, Ok((vec!['1', '2', '3'], \"\")));"] # [doc = " # }"] # [doc = " ```"] pub fn sep_end_by < F , Input , P , S > (parser : P , separator : S) -> SepEndBy < F , P , S > where Input : Stream , F : Extend < P :: Output > + Default , P : Parser < Input > , S : Parser < Input > , { SepEndBy { parser , separator , _marker : PhantomData , } }
};
}
