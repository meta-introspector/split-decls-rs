// Generated macro for macro_380 (macro)
macro_rules! Depcrate_parser_bytemacro_380 {
() => {
// Module: crate::parser::byte
// Provides: {"macro_380"}
// Dependencies: {}
parser ! { type PartialState = usize ; # [doc = " Zero-copy parser which reads a range of 0 or more tokens until `needle` is found."] # [doc = ""] # [doc = " If `a`, 'b' or `c` is not found, the parser will return an error."] # [doc = ""] # [doc = " Optimized variant of [`take_until_range`](../range/fn.take_until_range.html)"] # [doc = ""] # [doc = " ```"] # [doc = " use combine::*;"] # [doc = " use combine::parser::byte::take_until_bytes;"] # [doc = " assert_eq!("] # [doc = "     take_until_bytes(&b\"\\r\\n\"[..]).easy_parse(&b\"abc\\r\\n\"[..]).map(|(x, _)| x),"] # [doc = "     Ok((&b\"abc\"[..]))"] # [doc = " );"] # [doc = " // Also works on strings as long as `needle` is UTF-8"] # [doc = " assert_eq!("] # [doc = "     take_until_bytes(\"\\r\\n\".as_bytes()).easy_parse(\"abc\\r\\n\").map(|(x, _)| x),"] # [doc = "     Ok((\"abc\"))"] # [doc = " );"] # [doc = " ```"] pub fn take_until_bytes ['a , Input] (needle : &'a [u8]) (Input) -> Input :: Range where [Input : RangeStream , Input :: Range : AsRef < [u8] > + crate :: stream :: Range ,] { take_fn (move | haystack : Input :: Range | { let haystack = haystack . as_ref () ; match memslice (needle , haystack) { Some (i) => TakeRange :: Found (i) , None => TakeRange :: NotFound (haystack . len () . saturating_sub (needle . len () - 1)) , } }) } }
};
}
