// Generated macro for sep_by (function)
macro_rules! Depcrate_parser_repeatsep_by {
() => {
// Module: crate::parser::repeat
// Provides: {"sep_by"}
// Dependencies: {}
# [doc = " Parses `parser` zero or more time separated by `separator`, returning a collection with the"] # [doc = " values from `p`."] # [doc = ""] # [doc = " If the returned collection cannot be inferred type annotations must be supplied, either by"] # [doc = " annotating the resulting type binding `let collection: Vec<_> = ...` or by specializing when"] # [doc = " calling `sep_by`, `sep_by::<Vec<_>, _, _>(...)`."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # use combine::parser::char::digit;"] # [doc = " # fn main() {"] # [doc = " let mut parser = sep_by(digit(), token(','));"] # [doc = " let result_ok = parser.parse(\"1,2,3\");"] # [doc = " assert_eq!(result_ok, Ok((vec!['1', '2', '3'], \"\")));"] # [doc = " let result_ok2 = parser.parse(\"\");"] # [doc = " assert_eq!(result_ok2, Ok((vec![], \"\")));"] # [doc = " # }"] # [doc = " ```"] pub fn sep_by < F , Input , P , S > (parser : P , separator : S) -> SepBy < F , P , S > where Input : Stream , F : Extend < P :: Output > + Default , P : Parser < Input > , S : Parser < Input > , { SepBy { parser , separator , _marker : PhantomData , } }
};
}
