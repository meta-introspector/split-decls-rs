// Generated macro for ByteClass (struct)
macro_rules! DepcrateByteClass {
() => {
// Module: crate
// Provides: {"ByteClass"}
// Dependencies: {}
# [doc = " A byte class for byte ranges only."] # [doc = ""] # [doc = " A byte class has a canonical format that the parser guarantees. Its"] # [doc = " canonical format is defined by the following invariants:"] # [doc = ""] # [doc = " 1. Given any byte, it is matched by *at most* one byte range in a canonical"] # [doc = "    character class."] # [doc = " 2. Every adjacent byte range is separated by at least one byte."] # [doc = " 3. Given any pair of byte ranges `r1` and `r2`, if"] # [doc = "    `r1.end < r2.start`, then `r1` comes before `r2` in a canonical"] # [doc = "    character class."] # [doc = ""] # [doc = " In sum, any `ByteClass` produced by this crate's parser is a sorted"] # [doc = " sequence of non-overlapping ranges. This makes it possible to test whether"] # [doc = " a byte is matched by a class with a binary search."] # [doc = ""] # [doc = " If the case insensitive flag was set when parsing a character class,"] # [doc = " then simple ASCII-only case folding is done automatically. For example,"] # [doc = " `(?i)[a-c]` is automatically translated to `[a-cA-C]`."] # [derive (Clone , Debug , PartialEq , Eq)] pub struct ByteClass { ranges : Vec < ByteRange > , }
};
}
