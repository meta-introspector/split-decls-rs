// Generated macro for RegexSplitsN (struct)
macro_rules! Depcrate_re_unicodeRegexSplitsN {
() => {
// Module: crate::re_unicode
// Provides: {"RegexSplitsN"}
// Dependencies: {}
# [doc = " Yields at most `N` substrings delimited by a regular expression match."] # [doc = ""] # [doc = " The last substring will be whatever remains after splitting."] # [doc = ""] # [doc = " `'r` is the lifetime of the compiled regular expression and `'t` is the"] # [doc = " lifetime of the string being split."] pub struct RegexSplitsN < 'r , 't > { splits : RegexSplits < 'r , 't > , n : usize , }
};
}
