// Generated macro for find_many (function)
macro_rules! Depcrate_parser_regexfind_many {
() => {
// Module: crate::parser::regex
// Provides: {"find_many"}
// Dependencies: {}
# [doc = " Matches `regex` on the input by running `find_iter` on the input."] # [doc = " Returns all matches in a `F: FromIterator<Input::Range>`."] # [doc = " Consumes all input up until the end of the last match."] # [doc = ""] # [doc = " ```"] # [doc = " extern crate regex;"] # [doc = " extern crate combine;"] # [doc = " use regex::Regex;"] # [doc = " use regex::bytes;"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::regex::find_many;"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     let mut digits = find_many(Regex::new(\"[0-9]+\").unwrap());"] # [doc = "     assert_eq!(digits.parse(\"123 456 \"), Ok((vec![\"123\", \"456\"], \" \")));"] # [doc = "     assert_eq!(digits.parse(\"abc 123 456 \"), Ok((vec![\"123\", \"456\"], \" \")));"] # [doc = "     assert_eq!(digits.parse(\"abc\"), Ok((vec![], \"abc\")));"] # [doc = " }"] # [doc = " ```"] pub fn find_many < F , R , Input > (regex : R) -> FindMany < F , R , Input > where F : FromIterator < Input :: Range > , R : Regex < Input :: Range > , Input : RangeStream , Input :: Range : crate :: stream :: Range , { FindMany (regex , PhantomData) }
};
}
