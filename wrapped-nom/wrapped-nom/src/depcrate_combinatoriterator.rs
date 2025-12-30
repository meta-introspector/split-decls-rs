// Generated macro for iterator (function)
macro_rules! Depcrate_combinatoriterator {
() => {
// Module: crate::combinator
// Provides: {"iterator"}
// Dependencies: {}
# [doc = " Creates an iterator from input data and a parser."] # [doc = ""] # [doc = " Call the iterator's [ParserIterator::finish] method to get the remaining input if successful,"] # [doc = " or the error value if we encountered an error."] # [doc = ""] # [doc = " On [`Err::Error`], iteration will stop. To instead chain an error up, see [`cut`]."] # [doc = ""] # [doc = " ```rust"] # [doc = " use nom::{combinator::iterator, IResult, bytes::complete::tag, character::complete::alpha1, sequence::terminated};"] # [doc = " use std::collections::HashMap;"] # [doc = ""] # [doc = " let data = \"abc|defg|hijkl|mnopqr|123\";"] # [doc = " let mut it = iterator(data, terminated(alpha1, tag(\"|\")));"] # [doc = ""] # [doc = " let parsed = it.by_ref().map(|v| (v, v.len())).collect::<HashMap<_,_>>();"] # [doc = " let res: IResult<_,_> = it.finish();"] # [doc = ""] # [doc = " assert_eq!(parsed, [(\"abc\", 3usize), (\"defg\", 4), (\"hijkl\", 5), (\"mnopqr\", 6)].iter().cloned().collect());"] # [doc = " assert_eq!(res, Ok((\"123\", ())));"] # [doc = " ```"] pub fn iterator < Input , Error , F > (input : Input , f : F) -> ParserIterator < Input , Error , F > where F : Parser < Input > , Error : ParseError < Input > , { ParserIterator { iterator : f , input , state : Some (State :: Running) , } }
};
}
