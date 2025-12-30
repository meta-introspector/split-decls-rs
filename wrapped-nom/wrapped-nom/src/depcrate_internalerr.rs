// Generated macro for Err (enum)
macro_rules! Depcrate_internalErr {
() => {
// Module: crate::internal
// Provides: {"Err"}
// Dependencies: {}
# [doc = " The `Err` enum indicates the parser was not successful"] # [doc = ""] # [doc = " It has three cases:"] # [doc = ""] # [doc = " * `Incomplete` indicates that more data is needed to decide. The `Needed` enum"] # [doc = "   can contain how many additional bytes are necessary. If you are sure your parser"] # [doc = "   is working on full data, you can wrap your parser with the `complete` combinator"] # [doc = "   to transform that case in `Error`"] # [doc = " * `Error` means some parser did not succeed, but another one might (as an example,"] # [doc = "   when testing different branches of an `alt` combinator)"] # [doc = " * `Failure` indicates an unrecoverable error. For example, when a prefix has been"] # [doc = "   recognised and the next parser has been confirmed, if that parser fails, then the"] # [doc = "   entire process fails; there are no more parsers to try."] # [doc = ""] # [doc = " Distinguishing `Failure` this from `Error` is only relevant inside the parser's code. For"] # [doc = " external consumers, both mean that parsing failed."] # [doc = ""] # [doc = " See also: [`Finish`]."] # [doc = ""] # [derive (Debug , Clone , PartialEq)] pub enum Err < Failure , Error = Failure > { # [doc = " There was not enough data"] Incomplete (Needed) , # [doc = " The parser had an error (recoverable)"] Error (Error) , # [doc = " The parser had an unrecoverable error: we got to the right"] # [doc = " branch and we know other branches won't work, so backtrack"] # [doc = " as fast as possible"] Failure (Failure) , }
};
}
