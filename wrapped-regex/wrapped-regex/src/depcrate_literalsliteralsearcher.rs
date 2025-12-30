// Generated macro for LiteralSearcher (struct)
macro_rules! Depcrate_literalsLiteralSearcher {
() => {
// Module: crate::literals
// Provides: {"LiteralSearcher"}
// Dependencies: {}
# [doc = " A prefix extracted from a compiled regular expression."] # [doc = ""] # [doc = " A regex prefix is a set of literal strings that *must* be matched at the"] # [doc = " beginning of a regex in order for the entire regex to match."] # [doc = ""] # [doc = " There are a variety of ways to efficiently scan the search text for a"] # [doc = " prefix. Currently, there are three implemented:"] # [doc = ""] # [doc = " 1. The prefix is a single byte. Just use memchr."] # [doc = " 2. If the prefix is a set of two or more single byte prefixes, then"] # [doc = "    a single sparse map is created. Checking if there is a match is a lookup"] # [doc = "    in this map for each byte in the search text."] # [doc = " 3. In all other cases, build an Aho-Corasick automaton."] # [doc = ""] # [doc = " It's possible that there's room here for other substring algorithms,"] # [doc = " such as Boyer-Moore for single-set prefixes greater than 1, or Rabin-Karp"] # [doc = " for small sets of same-length prefixes."] # [derive (Clone , Debug)] pub struct LiteralSearcher { complete : bool , lcp : SingleSearch , lcs : SingleSearch , matcher : Matcher , }
};
}
