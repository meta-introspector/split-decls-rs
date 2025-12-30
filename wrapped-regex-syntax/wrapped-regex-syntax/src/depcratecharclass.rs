// Generated macro for CharClass (struct)
macro_rules! DepcrateCharClass {
() => {
// Module: crate
// Provides: {"CharClass"}
// Dependencies: {}
# [doc = " A character class."] # [doc = ""] # [doc = " A character class has a canonical format that the parser guarantees. Its"] # [doc = " canonical format is defined by the following invariants:"] # [doc = ""] # [doc = " 1. Given any Unicode scalar value, it is matched by *at most* one character"] # [doc = "    range in a canonical character class."] # [doc = " 2. Every adjacent character range is separated by at least one Unicode"] # [doc = "    scalar value."] # [doc = " 3. Given any pair of character ranges `r1` and `r2`, if"] # [doc = "    `r1.end < r2.start`, then `r1` comes before `r2` in a canonical"] # [doc = "    character class."] # [doc = ""] # [doc = " In sum, any `CharClass` produced by this crate's parser is a sorted"] # [doc = " sequence of non-overlapping ranges. This makes it possible to test whether"] # [doc = " a character is matched by a class with a binary search."] # [doc = ""] # [doc = " If the case insensitive flag was set when parsing a character class, then"] # [doc = " simple case folding is done automatically. For example, `(?i)[a-c]` is"] # [doc = " automatically translated to `[a-cA-C]`."] # [derive (Clone , Debug , PartialEq , Eq)] pub struct CharClass { ranges : Vec < ClassRange > , }
};
}
