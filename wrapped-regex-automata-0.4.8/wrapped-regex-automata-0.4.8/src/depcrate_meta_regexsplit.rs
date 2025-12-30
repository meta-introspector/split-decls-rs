// Generated macro for Split (struct)
macro_rules! Depcrate_meta_regexSplit {
() => {
// Module: crate::meta::regex
// Provides: {"Split"}
// Dependencies: {}
# [doc = " Yields all substrings delimited by a regular expression match."] # [doc = ""] # [doc = " The spans correspond to the offsets between matches."] # [doc = ""] # [doc = " The lifetime parameters are as follows:"] # [doc = ""] # [doc = " * `'r` represents the lifetime of the `Regex` that produced this iterator."] # [doc = " * `'h` represents the lifetime of the haystack being searched."] # [doc = ""] # [doc = " This iterator can be created with the [`Regex::split`] method."] # [derive (Debug)] pub struct Split < 'r , 'h > { finder : FindMatches < 'r , 'h > , last : usize , }
};
}
