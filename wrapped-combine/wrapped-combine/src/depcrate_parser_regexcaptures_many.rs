// Generated macro for captures_many (function)
macro_rules! Depcrate_parser_regexcaptures_many {
() => {
// Module: crate::parser::regex
// Provides: {"captures_many"}
// Dependencies: {}
# [doc = " Matches `regex` on the input by running `captures_iter` on the input."] # [doc = " Returns all captures which is part of the match in a `F: FromIterator<Input::Range>`."] # [doc = " Consumes all input up until the end of the last match."] # [doc = ""] # [doc = " ```"] # [doc = " extern crate regex;"] # [doc = " extern crate combine;"] # [doc = " use regex::Regex;"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::regex::captures_many;"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     let mut fields = captures_many(Regex::new(\"([a-z]+):([0-9]+)\").unwrap());"] # [doc = "     assert_eq!("] # [doc = "         fields.parse(\"test:123 field:456 \"),"] # [doc = "         Ok((vec![vec![\"test:123\", \"test\", \"123\"],"] # [doc = "                  vec![\"field:456\", \"field\", \"456\"]],"] # [doc = "             \" \""] # [doc = "         ))"] # [doc = "     );"] # [doc = "     assert_eq!("] # [doc = "         fields.parse(\"test:123 :456 \"),"] # [doc = "         Ok((vec![vec![\"test:123\", \"test\", \"123\"]],"] # [doc = "             \" :456 \""] # [doc = "         ))"] # [doc = "     );"] # [doc = " }"] # [doc = " ```"] pub fn captures_many < F , G , R , Input > (regex : R) -> CapturesMany < F , G , R , Input > where F : FromIterator < Input :: Range > , G : FromIterator < F > , R : Regex < Input :: Range > , Input : RangeStream , Input :: Range : crate :: stream :: Range , { CapturesMany (regex , PhantomData) }
};
}
