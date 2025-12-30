// Generated macro for RegexSplits (struct)
macro_rules! Depcrate_re_unicodeRegexSplits {
() => {
// Module: crate::re_unicode
// Provides: {"RegexSplits"}
// Dependencies: {}
# [doc = " Yields all substrings delimited by a regular expression match."] # [doc = ""] # [doc = " `'r` is the lifetime of the compiled regular expression and `'t` is the"] # [doc = " lifetime of the string being split."] pub struct RegexSplits < 'r , 't > { finder : FindMatches < 'r , 't > , last : usize , }
};
}
