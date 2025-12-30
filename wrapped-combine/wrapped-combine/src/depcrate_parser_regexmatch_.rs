// Generated macro for match_ (function)
macro_rules! Depcrate_parser_regexmatch_ {
() => {
// Module: crate::parser::regex
// Provides: {"match_"}
// Dependencies: {}
# [doc = " Matches `regex` on the input returning the entire input if it matches."] # [doc = " Never consumes any input."] # [doc = ""] # [doc = " ```"] # [doc = " extern crate regex;"] # [doc = " extern crate combine;"] # [doc = " use regex::Regex;"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::regex::match_;"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     let regex = Regex::new(\"[:alpha:]+\").unwrap();"] # [doc = "     assert_eq!("] # [doc = "         match_(&regex).parse(\"abc123\"),"] # [doc = "         Ok((\"abc123\", \"abc123\"))"] # [doc = "     );"] # [doc = " }"] # [doc = " ```"] pub fn match_ < R , Input > (regex : R) -> Match < R , Input > where R : Regex < Input :: Range > , Input : RangeStream , { Match (regex , PhantomData) }
};
}
