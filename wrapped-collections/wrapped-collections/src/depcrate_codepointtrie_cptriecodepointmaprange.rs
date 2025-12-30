// Generated macro for CodePointMapRange (struct)
macro_rules! Depcrate_codepointtrie_cptrieCodePointMapRange {
() => {
// Module: crate::codepointtrie::cptrie
// Provides: {"CodePointMapRange"}
// Dependencies: {}
# [doc = " Represents a range of consecutive code points sharing the same value in a"] # [doc = " code point map."] # [doc = ""] # [doc = " The start and end of the interval is represented as a"] # [doc = " `RangeInclusive<u32>`, and the value is represented as `T`."] # [derive (PartialEq , Eq , Debug , Clone)] pub struct CodePointMapRange < T > { # [doc = " Range of code points from start to end (inclusive)."] pub range : RangeInclusive < u32 > , # [doc = " Trie value associated with this range."] pub value : T , }
};
}
