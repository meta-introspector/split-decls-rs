// Generated macro for Hunk (struct)
macro_rules! DepcrateHunk {
() => {
// Module: crate
// Provides: {"Hunk"}
// Dependencies: {}
# [doc = " A single change in a `Diff` that represents a range of tokens (`before`)"] # [doc = " in the first sequence that were replaced by a different range of tokens"] # [doc = " in the second sequence (`after`)."] # [doc = ""] # [doc = " Tokens that are a"] # [derive (Debug , Clone , PartialEq , Eq , Hash , Default)] pub struct Hunk { pub before : Range < u32 > , pub after : Range < u32 > , }
};
}
