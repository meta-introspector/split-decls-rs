// Generated macro for Error (enum)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " An error that occurred during parsing or compiling a regular expression."] # [non_exhaustive] # [derive (Clone , PartialEq)] pub enum Error { # [doc = " A syntax error."] Syntax (String) , # [doc = " The compiled program exceeded the set size"] # [doc = " limit. The argument is the size limit imposed by"] # [doc = " [`RegexBuilder::size_limit`](crate::RegexBuilder::size_limit). Even"] # [doc = " when not configured explicitly, it defaults to a reasonable limit."] # [doc = ""] # [doc = " If you're getting this error, it occurred because your regex has been"] # [doc = " compiled to an intermediate state that is too big. It is important to"] # [doc = " note that exceeding this limit does _not_ mean the regex is too big to"] # [doc = " _work_, but rather, the regex is big enough that it may wind up being"] # [doc = " surprisingly slow when used in a search. In other words, this error is"] # [doc = " meant to be a practical heuristic for avoiding a performance footgun,"] # [doc = " and especially so for the case where the regex pattern is coming from"] # [doc = " an untrusted source."] # [doc = ""] # [doc = " There are generally two ways to move forward if you hit this error."] # [doc = " The first is to find some way to use a smaller regex. The second is to"] # [doc = " increase the size limit via `RegexBuilder::size_limit`. However, if"] # [doc = " your regex pattern is not from a trusted source, then neither of these"] # [doc = " approaches may be appropriate. Instead, you'll have to determine just"] # [doc = " how big of a regex you want to allow."] CompiledTooBig (usize) , }
};
}
